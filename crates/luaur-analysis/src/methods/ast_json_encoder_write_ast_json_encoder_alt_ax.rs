//! Source: `Analysis/src/AstJsonEncoder.cpp:676-687` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_error::AstExprError;
use luaur_ast::records::ast_node::AstNode;

impl AstJsonEncoder {
    pub fn write_ast_expr_error(&mut self, node: *mut AstExprError) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstExprError", |e| {
            e.write("expressions", &n.expressions);
            e.write("messageIndex", &n.message_index);
        });
    }
}
