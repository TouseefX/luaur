use crate::enums::autocomplete_context::AutocompleteContext;
use crate::records::autocomplete_result::AutocompleteResult;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use alloc::vec::Vec;
use luaur_ast::records::ast_node::AstNode;

impl AutocompleteResult {
    pub fn autocomplete_result_autocomplete_entry_map_vector_ast_node_autocomplete_context(
        entry_map: AutocompleteEntryMap,
        ancestry: Vec<*mut AstNode>,
        context: AutocompleteContext,
    ) -> Self {
        Self {
            entry_map,
            ancestry,
            context,
        }
    }
}
