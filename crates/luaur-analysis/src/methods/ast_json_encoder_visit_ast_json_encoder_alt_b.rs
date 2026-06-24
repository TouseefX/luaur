//! Source: `Analysis/src/AstJsonEncoder.cpp:1187-1198` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_singleton_bool::AstTypeSingletonBool;

impl AstJsonEncoder {
    pub fn visit_ast_type_singleton_bool(&mut self, node: *mut AstTypeSingletonBool) -> bool {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeSingletonBool", |e| {
            e.write("value", &n.value);
        });
        false
    }
}
