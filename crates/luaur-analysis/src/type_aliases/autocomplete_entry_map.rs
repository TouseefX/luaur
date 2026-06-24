use crate::records::autocomplete_entry::AutocompleteEntry;
use alloc::collections::BTreeMap;
use alloc::string::String;

pub type AutocompleteEntryMap = BTreeMap<String, AutocompleteEntry>;
