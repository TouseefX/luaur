//! Source: `Analysis/src/AstJsonEncoder.cpp:1113-1124` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_error::AstTypeError;

impl AstJsonEncoder {
    pub fn write_ast_type_error(&mut self, node: *mut AstTypeError) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeError", |e| {
            e.write("types", &n.types);
            e.write("messageIndex", &n.message_index);
        });
    }
}
