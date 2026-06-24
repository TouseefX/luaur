//! Source: `Analysis/src/AstJsonEncoder.cpp:595-606` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_unary::AstExprUnary;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_unary(&mut self, node: *mut AstExprUnary) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprUnary", |e| {
            e.write("op", &n.op);
            e.write("expr", &n.expr);
        });
    }
}
