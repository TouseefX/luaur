use crate::records::ast_visitor_tracking::AstVisitorTracking;
use luaur_ast::records::ast_node::AstNode;

impl AstVisitorTracking {
    pub fn operator_index(&mut self, index: usize) -> *mut AstNode {
        luaur_common::LUAU_ASSERT!(index < self.visited_nodes.len());

        self.seen.insert(index);
        self.visited_nodes[index]
    }
}
