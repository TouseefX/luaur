//! Source: `Analysis/src/AstJsonEncoder.cpp:1101-1111` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;

impl AstJsonEncoder {
    pub fn write_ast_type_intersection(&mut self, node: *mut AstTypeIntersection) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeIntersection", |e| {
            e.write("types", &n.types);
        });
    }
}
