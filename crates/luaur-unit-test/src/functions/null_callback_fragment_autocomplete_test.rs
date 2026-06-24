use alloc::string::String;
use luaur_analysis::records::extern_type::ExternType;
use luaur_analysis::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;

pub fn null_callback(
    _tag: String,
    _ptr: Option<*const ExternType>,
    _contents: Option<String>,
) -> Option<AutocompleteEntryMap> {
    None
}
