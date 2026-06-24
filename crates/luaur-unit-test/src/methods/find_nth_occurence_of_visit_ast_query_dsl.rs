use crate::records::find_nth_occurence_of::FindNthOccurenceOf;
use luaur_ast::records::ast_node::AstNode;

impl FindNthOccurenceOf {
    pub fn visit_ast_node(&mut self, n: *mut AstNode) -> bool {
        unsafe {
            luaur_ast::visit::ast_node_visit(n, self);
        }
        !self.the_node.is_null()
    }
}
