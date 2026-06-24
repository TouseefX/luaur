use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_break::AstStatBreak;

impl AstJsonEncoder {
    pub fn visit_ast_stat_break(&mut self, node: *mut AstStatBreak) -> bool {
        unsafe {
            self.write_ast_stat_break(node);
        }
        false
    }
}
