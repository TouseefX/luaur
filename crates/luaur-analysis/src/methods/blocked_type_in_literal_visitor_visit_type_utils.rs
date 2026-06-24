use crate::records::blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor;
use luaur_ast::records::ast_node::AstNode;

impl BlockedTypeInLiteralVisitor {
    pub fn visit_ast_node(&mut self, _node: *mut AstNode) -> bool {
        false
    }
}
