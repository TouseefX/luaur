use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::records::extern_type::ExternType;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use alloc::boxed::Box;
use alloc::string::String;
use core::option::Option;

pub type StringCompletionCallback =
    Box<dyn Fn(String, Option<*const ExternType>, Option<String>) -> Option<AutocompleteEntryMap>>;
