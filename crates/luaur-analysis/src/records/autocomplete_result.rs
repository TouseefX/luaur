use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use luaur_ast::records::ast_node::AstNode;

use crate::enums::autocomplete_context::AutocompleteContext;

#[derive(Debug, Clone)]
pub struct AutocompleteResult {
    pub entry_map: AutocompleteEntryMap,
    pub ancestry: alloc::vec::Vec<*mut AstNode>,
    pub context: AutocompleteContext,
}
