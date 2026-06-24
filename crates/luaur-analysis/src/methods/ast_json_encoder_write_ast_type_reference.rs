//! Source: `Analysis/src/AstJsonEncoder.cpp:992-1008` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_reference::AstTypeReference;

impl AstJsonEncoder {
    pub fn write_ast_type_reference(&mut self, node: *mut AstTypeReference) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeReference", |e| {
            if n.prefix.is_some() {
                e.write("prefix", &n.prefix);
            }
            if let Some(prefix_location) = n.prefix_location {
                e.write("prefixLocation", &prefix_location);
            }
            e.write("name", &n.name);
            e.write("nameLocation", &n.name_location);
            e.write("parameters", &n.parameters);
        });
    }
}
