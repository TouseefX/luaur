//! Source: `Analysis/src/AstJsonEncoder.cpp:971-982` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_error::AstStatError;

impl AstJsonEncoder {
    pub fn write_ast_stat_error(&mut self, node: *mut AstStatError) {
        let n = unsafe { &*node };
        self.write_node_ast_node_string_view_f(node as *mut AstNode, "AstStatError", |e| {
            e.write("expressions", &n.expressions);
            e.write("statements", &n.statements);
        });
    }
}
