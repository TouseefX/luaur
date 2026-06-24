use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
use luaur_ast::records::ast_type_pack::AstTypePack;

impl AutocompleteNodeFinder {
    pub fn visit_ast_type_pack(&mut self, _type_pack: *mut AstTypePack) -> bool {
        true
    }
}
