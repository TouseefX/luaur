//! Source: `Analysis/src/AstJsonEncoder.cpp:822-836` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;

impl AstJsonEncoder {
    pub fn write_ast_stat_for_in(&mut self, node: *mut AstStatForIn) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatForIn", |e| {
            e.write("vars", &n.vars);
            e.write("values", &n.values);
            e.write("body", &n.body);
            e.write("hasIn", &n.has_in);
            e.write("hasDo", &n.has_do);
        });
    }
}
