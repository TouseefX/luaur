//! Source: `Analysis/src/AstJsonEncoder.cpp:305-315` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_constant_number(&mut self, node: *mut AstExprConstantNumber) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(
            node as *mut AstNode,
            "AstExprConstantNumber",
            |e| {
                e.write("value", &n.value);
            },
        );
    }
}
