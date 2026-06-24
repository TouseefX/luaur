use crate::records::ast_type_visitor_tracking_withs::AstTypeVisitorTrackingWiths;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type::AstType;

impl AstTypeVisitorTrackingWiths {
    pub fn visit(&mut self, n: *mut AstType) -> bool {
        self.base.visit(n as *mut AstNode)
    }
}
