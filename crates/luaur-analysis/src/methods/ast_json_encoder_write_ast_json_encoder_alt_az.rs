//! Source: `Analysis/src/AstJsonEncoder.cpp:714-728` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_if::AstStatIf;

impl AstJsonEncoder {
    pub fn write_ast_stat_if(&mut self, node: *mut AstStatIf) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatIf", |e| {
            e.write("condition", &n.condition);
            e.write("thenbody", &n.thenbody);
            if !n.elsebody.is_null() {
                e.write("elsebody", &n.elsebody);
            }
            e.write("hasThen", &n.then_location.is_some());
        });
    }
}
