//! Source: `Analysis/src/AstJsonEncoder.cpp:541-555` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_if_else(&mut self, node: *mut AstExprIfElse) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprIfElse", |e| {
            e.write("condition", &n.condition);
            e.write("hasThen", &n.has_then);
            e.write("trueExpr", &n.true_expr);
            e.write("hasElse", &n.has_else);
            e.write("falseExpr", &n.false_expr);
        });
    }
}
