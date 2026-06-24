//! Source: `Analysis/src/AstJsonEncoder.cpp:649-661` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_binary(&mut self, node: *mut AstExprBinary) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprBinary", |e| {
            e.write("op", &n.op);
            e.write("left", &n.left);
            e.write("right", &n.right);
        });
    }
}
