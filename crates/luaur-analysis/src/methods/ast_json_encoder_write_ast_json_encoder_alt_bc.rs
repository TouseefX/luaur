use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_break::AstStatBreak;

impl AstJsonEncoder {
    pub fn write_ast_stat_break(&mut self, node: *mut AstStatBreak) {
        self.write_node_ast_node_string_view_f(node as *mut _, "AstStatBreak", |_| {});
    }
}
