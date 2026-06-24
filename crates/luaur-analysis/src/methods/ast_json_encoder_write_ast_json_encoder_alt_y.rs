//! Source: `Analysis/src/AstJsonEncoder.cpp:317-327` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_constant_integer::AstExprConstantInteger;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_constant_integer(&mut self, node: *mut AstExprConstantInteger) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(
            node as *mut AstNode,
            "AstExprConstantInteger",
            |e| {
                e.write("value", &n.value);
            },
        );
    }
}
