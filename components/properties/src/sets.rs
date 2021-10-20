// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The functions in this module return a [`UnicodeSet`] containing
//! the set of characters with a particular Unicode property.
//!
//! The descriptions of most properties are taken from [`TR44`], the documentation for the
//! Unicode Character Database.  Some properties are instead defined in [`TR18`], the
//! documentation for Unicode regular expressions. In particular, Annex C of this document
//! defines properties for POSIX compatibility.
//!
//! [`UnicodeSet`]: icu_uniset::UnicodeSet
//! [`TR44`]: https://www.unicode.org/reports/tr44
//! [`TR18`]: https://www.unicode.org/reports/tr18

use crate::provider::*;
use crate::*;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSetError;

type UnisetResult<'data> = Result<DataPayload<'data, UnicodePropertyV1Marker>, UnicodeSetError>;

// helper fn
fn get_uniset<'data, D>(provider: &D, resc_key: ResourceKey) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: resc_key,
            options: ResourceOptions {
                variant: None,
                langid: None,
            },
        },
    };

    let resp: DataResponse<UnicodePropertyV1Marker> = provider.load_payload(&data_req)?;

    let property_payload: DataPayload<UnicodePropertyV1Marker> = resp.take_payload()?;
    Ok(property_payload)
}

//
// Binary property getter fns
//

/// ASCII characters commonly used for the representation of hexadecimal numbers
pub fn get_ascii_hex_digit<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ASCII_HEX_DIGIT_V1)
}

/// Characters with the Alphabetic or Decimal_Number property
/// This is defined for POSIX compatibility.
pub fn get_alnum<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ALNUM_V1)
}

/// Alphabetic characters
pub fn get_alphabetic<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ALPHABETIC_V1)
}

/// Format control characters which have specific functions in the Unicode Bidirectional
/// Algorithm
pub fn get_bidi_control<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::BIDI_CONTROL_V1)
}

/// Characters that are mirrored in bidirectional text
pub fn get_bidi_mirrored<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::BIDI_MIRRORED_V1)
}

/// Horizontal whitespace characters
pub fn get_blank<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::BLANK_V1)
}

/// Uppercase, lowercase, and titlecase characters
pub fn get_cased<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CASED_V1)
}

/// Characters which are ignored for casing purposes
pub fn get_case_ignorable<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CASE_IGNORABLE_V1)
}

/// Characters that are excluded from composition
/// See <https://unicode.org/Public/UNIDATA/CompositionExclusions.txt>
pub fn get_full_composition_exclusion<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::FULL_COMPOSITION_EXCLUSION_V1)
}

/// Characters whose normalized forms are not stable under case folding
pub fn get_changes_when_casefolded<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_CASEFOLDED_V1)
}

/// Characters which may change when they undergo case mapping
pub fn get_changes_when_casemapped<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_CASEMAPPED_V1)
}

/// Characters which are not identical to their NFKC_Casefold mapping
pub fn get_changes_when_nfkc_casefolded<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_NFKC_CASEFOLDED_V1)
}

/// Characters whose normalized forms are not stable under a toLowercase mapping
pub fn get_changes_when_lowercased<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_LOWERCASED_V1)
}

/// Characters whose normalized forms are not stable under a toTitlecase mapping
pub fn get_changes_when_titlecased<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_TITLECASED_V1)
}

/// Characters whose normalized forms are not stable under a toUppercase mapping
pub fn get_changes_when_uppercased<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CHANGES_WHEN_UPPERCASED_V1)
}

/// Punctuation characters explicitly called out as dashes in the Unicode Standard, plus
/// their compatibility equivalents
pub fn get_dash<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::DASH_V1)
}

/// Deprecated characters. No characters will ever be removed from the standard, but the
/// usage of deprecated characters is strongly discouraged.
pub fn get_deprecated<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::DEPRECATED_V1)
}

/// For programmatic determination of default ignorable code points.  New characters that
/// should be ignored in rendering (unless explicitly supported) will be assigned in these
/// ranges, permitting programs to correctly handle the default rendering of such
/// characters when not otherwise supported.
pub fn get_default_ignorable_code_point<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::DEFAULT_IGNORABLE_CODE_POINT_V1)
}

/// Characters that linguistically modify the meaning of another character to which they apply
pub fn get_diacritic<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::DIACRITIC_V1)
}

/// Characters that can serve as a base for emoji modifiers
pub fn get_emoji_modifier_base<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_MODIFIER_BASE_V1)
}

/// Characters used in emoji sequences that normally do not appear on emoji keyboards as
/// separate choices, such as base characters for emoji keycaps
pub fn get_emoji_component<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_COMPONENT_V1)
}

/// Characters that are emoji modifiers
pub fn get_emoji_modifier<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_MODIFIER_V1)
}

/// Characters that are emoji
pub fn get_emoji<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_V1)
}

/// Characters that have emoji presentation by default
pub fn get_emoji_presentation<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EMOJI_PRESENTATION_V1)
}

/// Characters whose principal function is to extend the value of a preceding alphabetic
/// character or to extend the shape of adjacent characters.
pub fn get_extender<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EXTENDER_V1)
}

/// Pictographic symbols, as well as reserved ranges in blocks largely associated with
/// emoji characters
pub fn get_extended_pictographic<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::EXTENDED_PICTOGRAPHIC_V1)
}

/// Visible characters.
/// This is defined for POSIX compatibility.
pub fn get_graph<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::GRAPH_V1)
}

/// Property used together with the definition of Standard Korean Syllable Block to define
/// "Grapheme base". See D58 in Chapter 3, Conformance in the Unicode Standard.
pub fn get_grapheme_base<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::GRAPHEME_BASE_V1)
}

/// Property used to define "Grapheme extender". See D59 in Chapter 3, Conformance in the
/// Unicode Standard.
pub fn get_grapheme_extend<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::GRAPHEME_EXTEND_V1)
}

/// Deprecated property. Formerly proposed for programmatic determination of grapheme
/// cluster boundaries.
pub fn get_grapheme_link<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::GRAPHEME_LINK_V1)
}

/// Characters commonly used for the representation of hexadecimal numbers, plus their
/// compatibility equivalents
pub fn get_hex_digit<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::HEX_DIGIT_V1)
}

/// Deprecated property. Dashes which are used to mark connections between pieces of
/// words, plus the Katakana middle dot.
pub fn get_hyphen<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::HYPHEN_V1)
}

/// Characters that can come after the first character in an identifier. If using NFKC to
/// fold differences between characters, use [`get_xid_continue`] instead.  See
/// [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
/// more details.
pub fn get_id_continue<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ID_CONTINUE_V1)
}

/// Characters considered to be CJKV (Chinese, Japanese, Korean, and Vietnamese)
/// ideographs, or related siniform ideographs
pub fn get_ideographic<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::IDEOGRAPHIC_V1)
}

/// Characters that can begin an identifier. If using NFKC to fold differences between
/// characters, use [`get_xid_start`] instead.  See [`Unicode Standard Annex
/// #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.
pub fn get_id_start<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::ID_START_V1)
}

/// Characters used in Ideographic Description Sequences
pub fn get_ids_binary_operator<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::IDS_BINARY_OPERATOR_V1)
}

/// Characters used in Ideographic Description Sequences
pub fn get_ids_trinary_operator<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::IDS_TRINARY_OPERATOR_V1)
}

/// Format control characters which have specific functions for control of cursive joining
/// and ligation
pub fn get_join_control<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::JOIN_CONTROL_V1)
}

/// A small number of spacing vowel letters occurring in certain Southeast Asian scripts such as Thai and Lao
pub fn get_logical_order_exception<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::LOGICAL_ORDER_EXCEPTION_V1)
}

/// Lowercase characters
pub fn get_lowercase<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::LOWERCASE_V1)
}

/// Characters used in mathematical notation
pub fn get_math<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::MATH_V1)
}

/// Code points permanently reserved for internal use
pub fn get_noncharacter_code_point<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NONCHARACTER_CODE_POINT_V1)
}

/// Characters that are inert under NFC, i.e., they do not interact with adjacent characters
pub fn get_nfc_inert<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NFC_INERT_V1)
}

/// Characters that are inert under NFD, i.e., they do not interact with adjacent characters
pub fn get_nfd_inert<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NFD_INERT_V1)
}

/// Characters that are inert under NFKC, i.e., they do not interact with adjacent characters
pub fn get_nfkc_inert<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NFKC_INERT_V1)
}

/// Characters that are inert under NFKD, i.e., they do not interact with adjacent characters
pub fn get_nfkd_inert<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::NFKD_INERT_V1)
}

/// Characters used as syntax in patterns (such as regular expressions). See [`Unicode
/// Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
/// details.
pub fn get_pattern_syntax<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::PATTERN_SYNTAX_V1)
}

/// Characters used as whitespace in patterns (such as regular expressions).  See
/// [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
/// more details.
pub fn get_pattern_white_space<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::PATTERN_WHITE_SPACE_V1)
}

/// A small class of visible format controls, which precede and then span a sequence of
/// other characters, usually digits.
pub fn get_prepended_concatenation_mark<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::PREPENDED_CONCATENATION_MARK_V1)
}

/// Printable characters (visible characters and whitespace).
/// This is defined for POSIX compatibility.
pub fn get_print<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::PRINT_V1)
}

/// Punctuation characters that function as quotation marks.
pub fn get_quotation_mark<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::QUOTATION_MARK_V1)
}

/// Characters used in the definition of Ideographic Description Sequences
pub fn get_radical<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::RADICAL_V1)
}

/// Regional indicator characters, U+1F1E6..U+1F1FF
pub fn get_regional_indicator<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::REGIONAL_INDICATOR_V1)
}

/// Characters with a "soft dot", like i or j. An accent placed on these characters causes
/// the dot to disappear.
pub fn get_soft_dotted<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::SOFT_DOTTED_V1)
}

/// Characters that are starters in terms of Unicode normalization and combining character
/// sequences
pub fn get_segment_starter<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::SEGMENT_STARTER_V1)
}

/// Characters that are either the source of a case mapping or in the target of a case
/// mapping
pub fn get_case_sensitive<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::CASE_SENSITIVE_V1)
}

/// Punctuation characters that generally mark the end of sentences
pub fn get_sentence_terminal<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::SENTENCE_TERMINAL_V1)
}

/// Punctuation characters that generally mark the end of textual units
pub fn get_terminal_punctuation<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::TERMINAL_PUNCTUATION_V1)
}

/// A property which specifies the exact set of Unified CJK Ideographs in the standard
pub fn get_unified_ideograph<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::UNIFIED_IDEOGRAPH_V1)
}

/// Uppercase characters
pub fn get_uppercase<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::UPPERCASE_V1)
}

/// Characters that are Variation Selectors.
pub fn get_variation_selector<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::VARIATION_SELECTOR_V1)
}

/// Spaces, separator characters and other control characters which should be treated by
/// programming languages as "white space" for the purpose of parsing elements
pub fn get_white_space<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::WHITE_SPACE_V1)
}

/// Hexadecimal digits
/// This is defined for POSIX compatibility.
pub fn get_xdigit<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::XDIGIT_V1)
}

/// Characters that can begin an identifier.  See [`Unicode Standard Annex
/// #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.
pub fn get_xid_continue<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::XID_CONTINUE_V1)
}

/// Characters that can come after the first character in an identifier. See [`Unicode
/// Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
/// details.
pub fn get_xid_start<'data, D>(provider: &D) -> UnisetResult<'data>
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    get_uniset(provider, key::XID_START_V1)
}

//
// Enumerated property getter fns
//

/// Return a [`UnicodeSet`] for a particular value of the General_Category Unicode enumerated property
/// General_Category specifies enumerated Unicode general category types.
/// See <https://www.unicode.org/reports/tr44/> .
///
/// [`UnicodeSet`]: icu_uniset::UnicodeSet
pub fn get_for_general_category<'data, D>(
    provider: &'data D,
    enum_val: GeneralCategory,
) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    let key = match enum_val {
        GeneralCategory::Other => key::GENERAL_CATEGORY_OTHER_V1,
        GeneralCategory::Control => key::GENERAL_CATEGORY_CONTROL_V1,
        GeneralCategory::Format => key::GENERAL_CATEGORY_FORMAT_V1,
        GeneralCategory::Unassigned => key::GENERAL_CATEGORY_UNASSIGNED_V1,
        GeneralCategory::PrivateUse => key::GENERAL_CATEGORY_PRIVATE_USE_V1,
        GeneralCategory::Surrogate => key::GENERAL_CATEGORY_SURROGATE_V1,
        GeneralCategory::Letter => key::GENERAL_CATEGORY_LETTER_V1,
        GeneralCategory::CasedLetter => key::GENERAL_CATEGORY_CASED_LETTER_V1,
        GeneralCategory::LowercaseLetter => key::GENERAL_CATEGORY_LOWERCASE_LETTER_V1,
        GeneralCategory::ModifierLetter => key::GENERAL_CATEGORY_MODIFIER_LETTER_V1,
        GeneralCategory::OtherLetter => key::GENERAL_CATEGORY_OTHER_LETTER_V1,
        GeneralCategory::TitlecaseLetter => key::GENERAL_CATEGORY_TITLECASE_LETTER_V1,
        GeneralCategory::UppercaseLetter => key::GENERAL_CATEGORY_UPPERCASE_LETTER_V1,
        GeneralCategory::Mark => key::GENERAL_CATEGORY_MARK_V1,
        GeneralCategory::SpacingMark => key::GENERAL_CATEGORY_SPACING_MARK_V1,
        GeneralCategory::EnclosingMark => key::GENERAL_CATEGORY_ENCLOSING_MARK_V1,
        GeneralCategory::NonspacingMark => key::GENERAL_CATEGORY_NONSPACING_MARK_V1,
        GeneralCategory::Number => key::GENERAL_CATEGORY_NUMBER_V1,
        GeneralCategory::Digit => key::GENERAL_CATEGORY_DIGIT_V1,
        GeneralCategory::LetterNumber => key::GENERAL_CATEGORY_LETTER_NUMBER_V1,
        GeneralCategory::OtherNumber => key::GENERAL_CATEGORY_OTHER_NUMBER_V1,
        GeneralCategory::Punctuation => key::GENERAL_CATEGORY_PUNCTUATION_V1,
        GeneralCategory::ConnectorPunctuation => key::GENERAL_CATEGORY_CONNECTOR_PUNCTUATION_V1,
        GeneralCategory::DashPunctuation => key::GENERAL_CATEGORY_DASH_PUNCTUATION_V1,
        GeneralCategory::ClosePunctuation => key::GENERAL_CATEGORY_CLOSE_PUNCTUATION_V1,
        GeneralCategory::FinalPunctuation => key::GENERAL_CATEGORY_FINAL_PUNCTUATION_V1,
        GeneralCategory::InitialPunctuation => key::GENERAL_CATEGORY_INITIAL_PUNCTUATION_V1,
        GeneralCategory::OtherPunctuation => key::GENERAL_CATEGORY_OTHER_PUNCTUATION_V1,
        GeneralCategory::OpenPunctuation => key::GENERAL_CATEGORY_OPEN_PUNCTUATION_V1,
        GeneralCategory::Symbol => key::GENERAL_CATEGORY_SYMBOL_V1,
        GeneralCategory::CurrencySymbol => key::GENERAL_CATEGORY_CURRENCY_SYMBOL_V1,
        GeneralCategory::ModifierSymbol => key::GENERAL_CATEGORY_MODIFIER_SYMBOL_V1,
        GeneralCategory::MathSymbol => key::GENERAL_CATEGORY_MATH_SYMBOL_V1,
        GeneralCategory::OtherSymbol => key::GENERAL_CATEGORY_OTHER_SYMBOL_V1,
        GeneralCategory::Separator => key::GENERAL_CATEGORY_SEPARATOR_V1,
        GeneralCategory::LineSeparator => key::GENERAL_CATEGORY_LINE_SEPARATOR_V1,
        GeneralCategory::ParagraphSeparator => key::GENERAL_CATEGORY_PARAGRAPH_SEPARATOR_V1,
        GeneralCategory::SpaceSeparator => key::GENERAL_CATEGORY_SPACE_SEPARATOR_V1,
        _ => return Err(UnicodeSetError::UnknownGeneralCategorySet(enum_val.0)),
    };
    get_uniset(provider, key)
}

/// Return a [`UnicodeSet`] for a particular value of the Script Unicode enumerated property
/// See <https://www.unicode.org/reports/tr44/> .
///
/// [`UnicodeSet`]: icu_uniset::UnicodeSet
pub fn get_for_script<'data, D>(provider: &'data D, enum_val: Script) -> UnisetResult
where
    D: DataProvider<'data, UnicodePropertyV1Marker> + ?Sized,
{
    let key = match enum_val {
        Script::Adlam => key::SCRIPT_ADLAM_V1,
        Script::Ahom => key::SCRIPT_AHOM_V1,
        Script::AnatolianHieroglyphs => key::SCRIPT_ANATOLIAN_HIEROGLYPHS_V1,
        Script::Arabic => key::SCRIPT_ARABIC_V1,
        Script::Armenian => key::SCRIPT_ARMENIAN_V1,
        Script::Avestan => key::SCRIPT_AVESTAN_V1,
        Script::Balinese => key::SCRIPT_BALINESE_V1,
        Script::Bamum => key::SCRIPT_BAMUM_V1,
        Script::BassaVah => key::SCRIPT_BASSA_VAH_V1,
        Script::Batak => key::SCRIPT_BATAK_V1,
        Script::Bengali => key::SCRIPT_BENGALI_V1,
        Script::Bhaiksuki => key::SCRIPT_BHAIKSUKI_V1,
        Script::Bopomofo => key::SCRIPT_BOPOMOFO_V1,
        Script::Brahmi => key::SCRIPT_BRAHMI_V1,
        Script::Braille => key::SCRIPT_BRAILLE_V1,
        Script::Buginese => key::SCRIPT_BUGINESE_V1,
        Script::Buhid => key::SCRIPT_BUHID_V1,
        Script::CanadianAboriginal => key::SCRIPT_CANADIAN_ABORIGINAL_V1,
        Script::Carian => key::SCRIPT_CARIAN_V1,
        Script::CaucasianAlbanian => key::SCRIPT_CAUCASIAN_ALBANIAN_V1,
        Script::Chakma => key::SCRIPT_CHAKMA_V1,
        Script::Cham => key::SCRIPT_CHAM_V1,
        Script::Cherokee => key::SCRIPT_CHEROKEE_V1,
        Script::Chorasmian => key::SCRIPT_CHORASMIAN_V1,
        Script::Common => key::SCRIPT_COMMON_V1,
        Script::Coptic => key::SCRIPT_COPTIC_V1,
        Script::Cuneiform => key::SCRIPT_CUNEIFORM_V1,
        Script::Cypriot => key::SCRIPT_CYPRIOT_V1,
        Script::CyproMinoan => key::SCRIPT_CYPRO_MINOAN_V1,
        Script::Cyrillic => key::SCRIPT_CYRILLIC_V1,
        Script::Deseret => key::SCRIPT_DESERET_V1,
        Script::Devanagari => key::SCRIPT_DEVANAGARI_V1,
        Script::DivesAkuru => key::SCRIPT_DIVES_AKURU_V1,
        Script::Dogra => key::SCRIPT_DOGRA_V1,
        Script::Duployan => key::SCRIPT_DUPLOYAN_V1,
        Script::EgyptianHieroglyphs => key::SCRIPT_EGYPTIAN_HIEROGLYPHS_V1,
        Script::Elbasan => key::SCRIPT_ELBASAN_V1,
        Script::Elymaic => key::SCRIPT_ELYMAIC_V1,
        Script::Ethiopic => key::SCRIPT_ETHIOPIC_V1,
        Script::Georgian => key::SCRIPT_GEORGIAN_V1,
        Script::Glagolitic => key::SCRIPT_GLAGOLITIC_V1,
        Script::Gothic => key::SCRIPT_GOTHIC_V1,
        Script::Grantha => key::SCRIPT_GRANTHA_V1,
        Script::Greek => key::SCRIPT_GREEK_V1,
        Script::Gujarati => key::SCRIPT_GUJARATI_V1,
        Script::GunjalaGondi => key::SCRIPT_GUNJALA_GONDI_V1,
        Script::Gurmukhi => key::SCRIPT_GURMUKHI_V1,
        Script::Han => key::SCRIPT_HAN_V1,
        Script::Hangul => key::SCRIPT_HANGUL_V1,
        Script::HanifiRohingya => key::SCRIPT_HANIFI_ROHINGYA_V1,
        Script::Hanunoo => key::SCRIPT_HANUNOO_V1,
        Script::Hatran => key::SCRIPT_HATRAN_V1,
        Script::Hebrew => key::SCRIPT_HEBREW_V1,
        Script::Hiragana => key::SCRIPT_HIRAGANA_V1,
        Script::ImperialAramaic => key::SCRIPT_IMPERIAL_ARAMAIC_V1,
        Script::Inherited => key::SCRIPT_INHERITED_V1,
        Script::InscriptionalPahlavi => key::SCRIPT_INSCRIPTIONAL_PAHLAVI_V1,
        Script::InscriptionalParthian => key::SCRIPT_INSCRIPTIONAL_PARTHIAN_V1,
        Script::Javanese => key::SCRIPT_JAVANESE_V1,
        Script::Kaithi => key::SCRIPT_KAITHI_V1,
        Script::Kannada => key::SCRIPT_KANNADA_V1,
        Script::Katakana => key::SCRIPT_KATAKANA_V1,
        Script::KayahLi => key::SCRIPT_KAYAH_LI_V1,
        Script::Kharoshthi => key::SCRIPT_KHAROSHTHI_V1,
        Script::KhitanSmallScript => key::SCRIPT_KHITAN_SMALL_SCRIPT_V1,
        Script::Khmer => key::SCRIPT_KHMER_V1,
        Script::Khojki => key::SCRIPT_KHOJKI_V1,
        Script::Khudawadi => key::SCRIPT_KHUDAWADI_V1,
        Script::Lao => key::SCRIPT_LAO_V1,
        Script::Latin => key::SCRIPT_LATIN_V1,
        Script::Lepcha => key::SCRIPT_LEPCHA_V1,
        Script::Limbu => key::SCRIPT_LIMBU_V1,
        Script::LinearA => key::SCRIPT_LINEAR_A_V1,
        Script::LinearB => key::SCRIPT_LINEAR_B_V1,
        Script::Lisu => key::SCRIPT_LISU_V1,
        Script::Lycian => key::SCRIPT_LYCIAN_V1,
        Script::Lydian => key::SCRIPT_LYDIAN_V1,
        Script::Mahajani => key::SCRIPT_MAHAJANI_V1,
        Script::Makasar => key::SCRIPT_MAKASAR_V1,
        Script::Malayalam => key::SCRIPT_MALAYALAM_V1,
        Script::Mandaic => key::SCRIPT_MANDAIC_V1,
        Script::Manichaean => key::SCRIPT_MANICHAEAN_V1,
        Script::Marchen => key::SCRIPT_MARCHEN_V1,
        Script::MasaramGondi => key::SCRIPT_MASARAM_GONDI_V1,
        Script::Medefaidrin => key::SCRIPT_MEDEFAIDRIN_V1,
        Script::MeeteiMayek => key::SCRIPT_MEETEI_MAYEK_V1,
        Script::MendeKikakui => key::SCRIPT_MENDE_KIKAKUI_V1,
        Script::MeroiticCursive => key::SCRIPT_MEROITIC_CURSIVE_V1,
        Script::MeroiticHieroglyphs => key::SCRIPT_MEROITIC_HIEROGLYPHS_V1,
        Script::Miao => key::SCRIPT_MIAO_V1,
        Script::Modi => key::SCRIPT_MODI_V1,
        Script::Mongolian => key::SCRIPT_MONGOLIAN_V1,
        Script::Mro => key::SCRIPT_MRO_V1,
        Script::Multani => key::SCRIPT_MULTANI_V1,
        Script::Myanmar => key::SCRIPT_MYANMAR_V1,
        Script::Nabataean => key::SCRIPT_NABATAEAN_V1,
        Script::Nandinagari => key::SCRIPT_NANDINAGARI_V1,
        Script::NewTaiLue => key::SCRIPT_NEW_TAI_LUE_V1,
        Script::Newa => key::SCRIPT_NEWA_V1,
        Script::Nko => key::SCRIPT_NKO_V1,
        Script::Nushu => key::SCRIPT_NUSHU_V1,
        Script::NyiakengPuachueHmong => key::SCRIPT_NYIAKENG_PUACHUE_HMONG_V1,
        Script::Ogham => key::SCRIPT_OGHAM_V1,
        Script::OlChiki => key::SCRIPT_OL_CHIKI_V1,
        Script::OldHungarian => key::SCRIPT_OLD_HUNGARIAN_V1,
        Script::OldItalic => key::SCRIPT_OLD_ITALIC_V1,
        Script::OldNorthArabian => key::SCRIPT_OLD_NORTH_ARABIAN_V1,
        Script::OldPermic => key::SCRIPT_OLD_PERMIC_V1,
        Script::OldPersian => key::SCRIPT_OLD_PERSIAN_V1,
        Script::OldSogdian => key::SCRIPT_OLD_SOGDIAN_V1,
        Script::OldSouthArabian => key::SCRIPT_OLD_SOUTH_ARABIAN_V1,
        Script::OldTurkic => key::SCRIPT_OLD_TURKIC_V1,
        Script::OldUyghur => key::SCRIPT_OLD_UYGHUR_V1,
        Script::Oriya => key::SCRIPT_ORIYA_V1,
        Script::Osage => key::SCRIPT_OSAGE_V1,
        Script::Osmanya => key::SCRIPT_OSMANYA_V1,
        Script::PahawhHmong => key::SCRIPT_PAHAWH_HMONG_V1,
        Script::Palmyrene => key::SCRIPT_PALMYRENE_V1,
        Script::PauCinHau => key::SCRIPT_PAU_CIN_HAU_V1,
        Script::PhagsPa => key::SCRIPT_PHAGS_PA_V1,
        Script::Phoenician => key::SCRIPT_PHOENICIAN_V1,
        Script::PsalterPahlavi => key::SCRIPT_PSALTER_PAHLAVI_V1,
        Script::Rejang => key::SCRIPT_REJANG_V1,
        Script::Runic => key::SCRIPT_RUNIC_V1,
        Script::Samaritan => key::SCRIPT_SAMARITAN_V1,
        Script::Saurashtra => key::SCRIPT_SAURASHTRA_V1,
        Script::Sharada => key::SCRIPT_SHARADA_V1,
        Script::Shavian => key::SCRIPT_SHAVIAN_V1,
        Script::Siddham => key::SCRIPT_SIDDHAM_V1,
        Script::SignWriting => key::SCRIPT_SIGNWRITING_V1,
        Script::Sinhala => key::SCRIPT_SINHALA_V1,
        Script::Sogdian => key::SCRIPT_SOGDIAN_V1,
        Script::SoraSompeng => key::SCRIPT_SORA_SOMPENG_V1,
        Script::Soyombo => key::SCRIPT_SOYOMBO_V1,
        Script::Sundanese => key::SCRIPT_SUNDANESE_V1,
        Script::SylotiNagri => key::SCRIPT_SYLOTI_NAGRI_V1,
        Script::Syriac => key::SCRIPT_SYRIAC_V1,
        Script::Tagalog => key::SCRIPT_TAGALOG_V1,
        Script::Tagbanwa => key::SCRIPT_TAGBANWA_V1,
        Script::TaiLe => key::SCRIPT_TAI_LE_V1,
        Script::TaiTham => key::SCRIPT_TAI_THAM_V1,
        Script::TaiViet => key::SCRIPT_TAI_VIET_V1,
        Script::Takri => key::SCRIPT_TAKRI_V1,
        Script::Tamil => key::SCRIPT_TAMIL_V1,
        Script::Tangsa => key::SCRIPT_TANGSA_V1,
        Script::Tangut => key::SCRIPT_TANGUT_V1,
        Script::Telugu => key::SCRIPT_TELUGU_V1,
        Script::Thaana => key::SCRIPT_THAANA_V1,
        Script::Thai => key::SCRIPT_THAI_V1,
        Script::Tibetan => key::SCRIPT_TIBETAN_V1,
        Script::Tifinagh => key::SCRIPT_TIFINAGH_V1,
        Script::Tirhuta => key::SCRIPT_TIRHUTA_V1,
        Script::Toto => key::SCRIPT_TOTO_V1,
        Script::Ugaritic => key::SCRIPT_UGARITIC_V1,
        Script::Unknown => key::SCRIPT_UNKNOWN_V1,
        Script::Vai => key::SCRIPT_VAI_V1,
        Script::Vithkuqi => key::SCRIPT_VITHKUQI_V1,
        Script::Wancho => key::SCRIPT_WANCHO_V1,
        Script::WarangCiti => key::SCRIPT_WARANG_CITI_V1,
        Script::Yezidi => key::SCRIPT_YEZIDI_V1,
        Script::Yi => key::SCRIPT_YI_V1,
        Script::ZanabazarSquare => key::SCRIPT_ZANABAZAR_SQUARE_V1,
        _ => return Err(UnicodeSetError::UnknownScriptId(enum_val.0)),
    };
    get_uniset(provider, key)
}