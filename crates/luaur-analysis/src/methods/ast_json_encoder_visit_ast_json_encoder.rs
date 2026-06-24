//! Source: `Analysis/src/AstJsonEncoder.cpp:1174-1185` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_group::AstTypeGroup;

impl AstJsonEncoder {
    pub fn visit_ast_type_group(&mut self, node: *mut AstTypeGroup) -> bool {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstTypeGroup", |e| {
            e.write("inner", &n.type_);
        });
        false
    }
}
