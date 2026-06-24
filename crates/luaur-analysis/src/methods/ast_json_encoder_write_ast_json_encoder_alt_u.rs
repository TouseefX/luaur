//! Source: `Analysis/src/AstJsonEncoder.cpp:276-286` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_group(&mut self, node: *mut AstExprGroup) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprGroup", |e| {
            e.write("expr", &n.expr);
        });
    }
}
