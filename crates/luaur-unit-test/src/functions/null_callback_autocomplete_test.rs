use alloc::string::String;
use luaur_analysis::records::extern_type::ExternType;
use luaur_analysis::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;

pub fn null_callback(
    tag: String,
    ptr: Option<*const ExternType>,
    contents: Option<String>,
) -> Option<AutocompleteEntryMap> {
    None
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use null_callback as null_callback_mut;
