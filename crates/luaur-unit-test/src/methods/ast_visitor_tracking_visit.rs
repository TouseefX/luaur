use crate::records::ast_visitor_tracking::AstVisitorTracking;
use luaur_ast::records::ast_node::AstNode;

impl AstVisitorTracking {
    pub fn visit(&mut self, n: *mut AstNode) -> bool {
        self.visited_nodes.push(n);
        true
    }
}
