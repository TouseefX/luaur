//! Source: `Analysis/src/AstJsonEncoder.cpp:804-820` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_for::AstStatFor;

impl AstJsonEncoder {
    pub fn write_ast_stat_for(&mut self, node: *mut AstStatFor) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatFor", |e| {
            e.write("var", &n.var);
            e.write("from", &n.from);
            e.write("to", &n.to);
            if !n.step.is_null() {
                e.write("step", &n.step);
            }
            e.write("body", &n.body);
            e.write("hasDo", &n.has_do);
        });
    }
}
