//! Source: `Analysis/src/AstJsonEncoder.cpp:663-674` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_type_assertion(&mut self, node: *mut AstExprTypeAssertion) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprTypeAssertion", |e| {
            e.write("expr", &n.expr);
            e.write("annotation", &n.annotation);
        });
    }
}
