//! Source: `Analysis/src/AstJsonEncoder.cpp:744-755` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;

impl AstJsonEncoder {
    pub fn write_ast_stat_repeat(&mut self, node: *mut AstStatRepeat) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatRepeat", |e| {
            e.write("condition", &n.condition);
            e.write("body", &n.body);
        });
    }
}
