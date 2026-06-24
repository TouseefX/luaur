//! Source: `Analysis/src/AstJsonEncoder.cpp:1200-1211` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_singleton_string::AstTypeSingletonString;

impl AstJsonEncoder {
    pub fn visit_ast_type_singleton_string(&mut self, node: *mut AstTypeSingletonString) -> bool {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(
            node as *mut AstNode,
            "AstTypeSingletonString",
            |e| {
                e.write("value", &n.value);
            },
        );
        false
    }
}
