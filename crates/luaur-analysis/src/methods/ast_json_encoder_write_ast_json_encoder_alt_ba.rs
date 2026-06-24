//! Source: `Analysis/src/AstJsonEncoder.cpp:730-742` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_while::AstStatWhile;

impl AstJsonEncoder {
    pub fn write_ast_stat_while(&mut self, node: *mut AstStatWhile) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatWhile", |e| {
            e.write("condition", &n.condition);
            e.write("body", &n.body);
            e.write("hasDo", &n.has_do);
        });
    }
}
