//! Source: `Analysis/src/AstJsonEncoder.cpp:424-435` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_index_expr(&mut self, node: *mut AstExprIndexExpr) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprIndexExpr", |e| {
            e.write("expr", &n.expr);
            e.write("index", &n.index);
        });
    }
}
