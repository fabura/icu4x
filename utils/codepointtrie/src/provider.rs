// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::yoke::{self, Yokeable, ZeroCopyFrom};

/// A map efficiently storing data about individual characters.
#[derive(Debug, Eq, PartialEq, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct UnicodePropertyMapV1<'data, T: TrieValue> {
    /// A codepoint trie storing the data
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub codepoint_trie: CodePointTrie<'data, T>,
}

impl<'data, T: TrieValue> Clone for UnicodePropertyMapV1<'data, T>
where
    <T as zerovec::ule::AsULE>::ULE: Clone,
{
    fn clone(&self) -> Self {
        UnicodePropertyMapV1 {
            codepoint_trie: self.codepoint_trie.clone(),
        }
    }
}

/// Marker type for UnicodePropertyMapV1.
/// This is generated by hand because icu_provider::data_struct doesn't support generics yet.
pub struct UnicodePropertyMapV1Marker<T: TrieValue> {
    _phantom: core::marker::PhantomData<T>,
}

impl<'data, T: TrieValue> icu_provider::DataMarker<'data> for UnicodePropertyMapV1Marker<T> {
    type Yokeable = UnicodePropertyMapV1<'static, T>;
    type Cart = UnicodePropertyMapV1<'data, T>;
}
