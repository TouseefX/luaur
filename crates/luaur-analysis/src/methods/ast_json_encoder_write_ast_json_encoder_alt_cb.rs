//! Source: `Analysis/src/AstJsonEncoder.cpp:1089-1099` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_union::AstTypeUnion;

impl AstJsonEncoder {
    pub fn write_ast_type_union(&mut self, node: *mut AstTypeUnion) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeUnion", |e| {
            e.write("types", &n.types);
        });
    }
}
