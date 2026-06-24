//! Source: `Analysis/src/AstJsonEncoder.cpp:779-789` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_expr::AstStatExpr;

impl AstJsonEncoder {
    pub fn write_ast_stat_expr(&mut self, node: *mut AstStatExpr) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatExpr", |e| {
            e.write("expr", &n.expr);
        });
    }
}
