use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

impl AutocompleteNodeFinder {
    pub fn visit_ast_type(&mut self, type_: *mut AstType) -> bool {
        let location = unsafe { (*type_).base.location };
        if location.begin < self.pos && self.pos <= location.end {
            self.ancestry
                .push(type_ as *mut luaur_ast::records::ast_node::AstNode);
            return true;
        }
        false
    }
}
