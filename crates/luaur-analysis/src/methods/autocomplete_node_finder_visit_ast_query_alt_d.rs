use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_error::AstTypeError;

impl AutocompleteNodeFinder {
    pub fn visit_ast_type_error(&mut self, type_: *mut AstTypeError) -> bool {
        // For a missing type, match the whole range including the start position
        let type_ref = unsafe { &*type_ };
        if type_ref.is_missing && type_ref.base.base.location.containsClosed(self.pos) {
            self.ancestry.push(type_ as *mut AstNode);
            return true;
        }
        false
    }
}
