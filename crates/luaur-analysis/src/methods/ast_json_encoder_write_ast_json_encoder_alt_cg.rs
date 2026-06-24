//! Source: `Analysis/src/AstJsonEncoder.cpp:1150-1160` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_pack_generic::AstTypePackGeneric;

impl AstJsonEncoder {
    pub fn write_ast_type_pack_generic(&mut self, node: *mut AstTypePackGeneric) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypePackGeneric", |e| {
            e.write("genericName", &n.generic_name);
        });
    }
}
