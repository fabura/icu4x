// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date;
use crate::error::DateTimeFormatError;
use crate::fields;
use crate::options::{components, length, preferences, DateTimeFormatOptions};
use crate::pattern::{
    hour_cycle,
    reference::{Pattern, PatternPlurals},
};
use crate::provider;
use crate::provider::gregory::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker};
use crate::skeleton;
use alloc::borrow::Cow;
use icu_locid::Locale;
use icu_provider::prelude::*;

type Result<T> = core::result::Result<T, DateTimeFormatError>;

/// This function is used to select appropriate pattern from data provider
/// data for the given options and locale.
///
/// It uses a temporary structure `PatternSelector` to lazily load data as needed
/// as it traverses the decision tree based on the provided options.
pub(crate) fn patterns_for_options<'data, D>(
    data_provider: &D,
    locale: &Locale,
    options: &DateTimeFormatOptions,
) -> Result<Option<PatternPlurals>>
where
    D: DataProvider<'data, DatePatternsV1Marker>
        + DataProvider<'data, DateSkeletonPatternsV1Marker>,
{
    let mut selector = PatternSelector::new(data_provider, locale);
    selector.patterns_for_options(options)
}

/// Private temporary structure used to cache lazily loaded data from the data provider.
///
/// The structure takes a reference to data provider and locale, and for given
/// options loads the appropriate data and selects the appropriate pattern.
///
/// This is used by all public structures such as `DateTimeFormat` and `ZonedDateTimeFormat`.
///
/// # Design Decisions
///
/// This structure is acts at a junction of data provider and options bags.
/// It allows us to chunk data into small payloads, selectively
/// load them when needed and cache for the duration of the selection.
///
/// # Implementation Details
///
/// Because of how Rust borrow checking works, we use new type structs for each payload option
/// to allow for mutable operations on each field separately.
///
/// The content of `retrieve` method seem like it would work with `Option::get_or_insert_with` but
/// must be falliable.
pub struct PatternSelector<'a, 'data, D> {
    data_provider: &'a D,
    locale: &'a Locale,
    date_patterns: DatePatternsOption<'data>,
    skeletons: DateSkeletonPatternsOption<'data>,
}

#[derive(Default)]
struct DatePatternsOption<'data>(Option<DataPayload<'data, DatePatternsV1Marker>>);

impl<'data> DatePatternsOption<'data> {
    fn retrieve<D>(
        &mut self,
        data_provider: &D,
        locale: &Locale,
    ) -> Result<&DataPayload<'data, DatePatternsV1Marker>>
    where
        D: DataProvider<'data, DatePatternsV1Marker>,
    {
        if let Some(ref value) = self.0 {
            Ok(value)
        } else {
            let patterns_data = data_provider
                .load_payload(&DataRequest {
                    resource_path: ResourcePath {
                        key: provider::key::GREGORY_DATE_PATTERNS_V1,
                        options: ResourceOptions {
                            variant: None,
                            langid: Some(locale.clone().into()),
                        },
                    },
                })?
                .take_payload()?;
            Ok(self.0.insert(patterns_data))
        }
    }
}

#[derive(Default)]
struct DateSkeletonPatternsOption<'data>(Option<DataPayload<'data, DateSkeletonPatternsV1Marker>>);

impl<'data> DateSkeletonPatternsOption<'data> {
    fn retrieve<D>(
        &mut self,
        data_provider: &D,
        locale: &Locale,
    ) -> Result<&DataPayload<'data, DateSkeletonPatternsV1Marker>>
    where
        D: DataProvider<'data, DateSkeletonPatternsV1Marker>,
    {
        if let Some(ref value) = self.0 {
            Ok(value)
        } else {
            let patterns_data = data_provider
                .load_payload(&DataRequest {
                    resource_path: ResourcePath {
                        key: provider::key::GREGORY_DATE_SKELETON_PATTERNS_V1,
                        options: ResourceOptions {
                            variant: None,
                            langid: Some(locale.clone().into()),
                        },
                    },
                })?
                .take_payload()?;
            Ok(self.0.insert(patterns_data))
        }
    }
}

impl<'a, 'data, D> PatternSelector<'a, 'data, D>
where
    D: DataProvider<'data, DatePatternsV1Marker>
        + DataProvider<'data, DateSkeletonPatternsV1Marker>,
{
    /// Create a new `PatternSelector` for the given data provider and locale.
    fn new(data_provider: &'a D, locale: &'a Locale) -> Self {
        Self {
            data_provider,
            locale,
            date_patterns: DatePatternsOption::default(),
            skeletons: DateSkeletonPatternsOption::default(),
        }
    }

    /// Determine the appropriate `PatternPlurals` for the given options and data from the data provider.
    fn patterns_for_options(
        &mut self,
        options: &DateTimeFormatOptions,
    ) -> Result<Option<PatternPlurals>> {
        match options {
            DateTimeFormatOptions::Length(bag) => self
                .pattern_for_length_bag(bag)
                .map(|opt_pattern| opt_pattern.map(|pattern| pattern.into())),
            DateTimeFormatOptions::Components(bag) => self.patterns_for_components_bag(bag),
        }
    }

    /// Determine the appropriate `Pattern` for a given `options::Length` bag.
    fn pattern_for_length_bag(&mut self, length: &length::Bag) -> Result<Option<Pattern>> {
        match (length.date, length.time) {
            (None, None) => Ok(None),
            (None, Some(time_length)) => self
                .pattern_for_time_length(time_length, &length.preferences)
                .map(Some),
            (Some(date_length), None) => self.pattern_for_date_length(date_length).map(Some),
            (Some(date_length), Some(time_length)) => {
                let time = self.pattern_for_time_length(time_length, &length.preferences)?;
                let date = self.pattern_for_date_length(date_length)?;

                self.pattern_for_datetime_length(date_length, date, time)
                    .map(Some)
            }
        }
    }

    /// Determine the appropriate `Pattern` for a given `options::length::Date` bag.
    fn pattern_for_date_length(&mut self, length: length::Date) -> Result<Pattern> {
        let date = &self
            .date_patterns
            .retrieve(self.data_provider, self.locale)?
            .get()
            .date;
        let s = match length {
            length::Date::Full => &date.full,
            length::Date::Long => &date.long,
            length::Date::Medium => &date.medium,
            length::Date::Short => &date.short,
        };
        Ok(Pattern::from_bytes(s)?)
    }

    /// Determine the appropriate `Pattern` for a given `options::length::Time` bag.
    /// If a preference for an hour cycle is set, it will look look up a pattern in the time_h11_12 or
    /// time_h23_h24 provider data, and then manually modify the symbol in the pattern if needed.
    fn pattern_for_time_length(
        &mut self,
        length: length::Time,
        preferences: &Option<preferences::Bag>,
    ) -> Result<Pattern> {
        let patterns = &self
            .date_patterns
            .retrieve(self.data_provider, self.locale)?
            .get();
        // Determine the coarse hour cycle patterns to use from either the preference bag,
        // or the preferred hour cycle for the locale.
        let time = if let Some(preferences::Bag {
            hour_cycle: Some(hour_cycle_pref),
        }) = preferences
        {
            match hour_cycle_pref {
                preferences::HourCycle::H11 | preferences::HourCycle::H12 => &patterns.time_h11_h12,
                preferences::HourCycle::H23 | preferences::HourCycle::H24 => &patterns.time_h23_h24,
            }
        } else {
            match patterns.preferred_hour_cycle {
                crate::pattern::CoarseHourCycle::H11H12 => &patterns.time_h11_h12,
                crate::pattern::CoarseHourCycle::H23H24 => &patterns.time_h23_h24,
            }
        };

        let mut pattern = Pattern::from_bytes(match length {
            length::Time::Full => &time.full,
            length::Time::Long => &time.long,
            length::Time::Medium => &time.medium,
            length::Time::Short => &time.short,
        })?;

        hour_cycle::naively_apply_preferences(&mut pattern, preferences);

        Ok(pattern)
    }

    /// Determine the appropriate `Pattern` for a given `options::length::Date` and
    /// `options::length::Time` bag.
    fn pattern_for_datetime_length(
        &mut self,
        date_time_length: length::Date,
        date: Pattern,
        time: Pattern,
    ) -> Result<Pattern> {
        let patterns = &self
            .date_patterns
            .retrieve(self.data_provider, self.locale)?
            .get();
        let s = match date_time_length {
            length::Date::Full => &patterns.length_combinations.full,
            length::Date::Long => &patterns.length_combinations.long,
            length::Date::Medium => &patterns.length_combinations.medium,
            length::Date::Short => &patterns.length_combinations.short,
        };
        Ok(Pattern::from_bytes_combination(s, date, time)?)
    }

    /// Determine the appropriate `PatternPlurals` for a given `options::components::Bag`.
    fn patterns_for_components_bag(
        &mut self,
        components: &components::Bag,
    ) -> Result<Option<PatternPlurals>> {
        let patterns = &self
            .date_patterns
            .retrieve(self.data_provider, self.locale)?
            .get();
        let skeletons = &self
            .skeletons
            .retrieve(self.data_provider, self.locale)?
            .get();
        // Not all skeletons are currently supported.
        let requested_fields = components.to_vec_fields();
        Ok(
            match skeleton::create_best_pattern_for_fields(
                skeletons,
                &patterns.length_combinations,
                &requested_fields,
                components,
                false, // Prefer the requested fields over the matched pattern.
            ) {
                skeleton::BestSkeleton::AllFieldsMatch(pattern)
                | skeleton::BestSkeleton::MissingOrExtraFields(pattern) => Some(pattern.0),
                skeleton::BestSkeleton::NoMatch => None,
            },
        )
    }
}

pub trait DateTimeSymbols {
    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> &Cow<str>;
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
    ) -> &Cow<str>;
    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> &Cow<str>;
}

impl DateTimeSymbols for provider::gregory::DateSymbolsV1 {
    fn get_symbol_for_weekday(
        &self,
        weekday: fields::Weekday,
        length: fields::FieldLength,
        day: date::IsoWeekday,
    ) -> &Cow<str> {
        let widths = match weekday {
            fields::Weekday::Format => &self.weekdays.format,
            fields::Weekday::StandAlone => {
                if let Some(ref widths) = self.weekdays.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        fields::FieldLength::Six => widths
                            .short
                            .as_ref()
                            .or_else(|| widths.abbreviated.as_ref()),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return &symbols.0[(day as usize) % 7];
                    } else {
                        return self.get_symbol_for_weekday(fields::Weekday::Format, length, day);
                    }
                } else {
                    return self.get_symbol_for_weekday(fields::Weekday::Format, length, day);
                }
            }
            fields::Weekday::Local => unimplemented!(),
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            fields::FieldLength::Six => widths.short.as_ref().unwrap_or(&widths.abbreviated),
            _ => &widths.abbreviated,
        };
        &symbols.0[(day as usize) % 7]
    }

    fn get_symbol_for_month(
        &self,
        month: fields::Month,
        length: fields::FieldLength,
        num: usize,
    ) -> &Cow<str> {
        // TODO(#493): Support symbols for non-Gregorian calendars.
        debug_assert!(num < 12);
        let widths = match month {
            fields::Month::Format => &self.months.format,
            fields::Month::StandAlone => {
                if let Some(ref widths) = self.months.stand_alone {
                    let symbols = match length {
                        fields::FieldLength::Wide => widths.wide.as_ref(),
                        fields::FieldLength::Narrow => widths.narrow.as_ref(),
                        _ => widths.abbreviated.as_ref(),
                    };
                    if let Some(symbols) = symbols {
                        return &symbols.0[num];
                    } else {
                        return self.get_symbol_for_month(fields::Month::Format, length, num);
                    }
                } else {
                    return self.get_symbol_for_month(fields::Month::Format, length, num);
                }
            }
        };
        let symbols = match length {
            fields::FieldLength::Wide => &widths.wide,
            fields::FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        &symbols.0[num]
    }

    fn get_symbol_for_day_period(
        &self,
        day_period: fields::DayPeriod,
        length: fields::FieldLength,
        hour: date::IsoHour,
        is_top_of_hour: bool,
    ) -> &Cow<str> {
        use fields::{DayPeriod::NoonMidnight, FieldLength};
        let widths = &self.day_periods.format;
        let symbols = match length {
            FieldLength::Wide => &widths.wide,
            FieldLength::Narrow => &widths.narrow,
            _ => &widths.abbreviated,
        };
        match (day_period, u8::from(hour), is_top_of_hour) {
            (NoonMidnight, 00, true) => symbols.midnight.as_ref().unwrap_or(&symbols.am),
            (NoonMidnight, 12, true) => symbols.noon.as_ref().unwrap_or(&symbols.pm),
            (_, hour, _) if hour < 12 => &symbols.am,
            _ => &symbols.pm,
        }
    }
}
