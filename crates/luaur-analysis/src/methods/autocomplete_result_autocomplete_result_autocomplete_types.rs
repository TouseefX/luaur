use crate::enums::autocomplete_context::AutocompleteContext;
use crate::records::autocomplete_result::AutocompleteResult;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;

impl AutocompleteResult {
    pub fn autocomplete_result() -> Self {
        Self::autocomplete_result_autocomplete_entry_map_vector_ast_node_autocomplete_context(
            AutocompleteEntryMap::default(),
            alloc::vec::Vec::new(),
            AutocompleteContext::Unknown,
        )
    }
}
