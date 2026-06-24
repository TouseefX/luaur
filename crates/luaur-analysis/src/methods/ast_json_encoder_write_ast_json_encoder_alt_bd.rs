use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_continue::AstStatContinue;

impl AstJsonEncoder {
    pub fn write_ast_stat_continue(&mut self, node: *mut AstStatContinue) {
        self.write_node_ast_node_string_view_f(
            node as *mut luaur_ast::records::ast_node::AstNode,
            "AstStatContinue",
            |_| {},
        );
    }
}
