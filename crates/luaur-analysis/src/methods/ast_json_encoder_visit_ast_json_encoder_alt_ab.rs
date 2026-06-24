use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_continue::AstStatContinue;

impl AstJsonEncoder {
    pub fn visit_ast_stat_continue(&mut self, node: *mut AstStatContinue) -> bool {
        unsafe {
            self.write_ast_stat_continue(node);
        }
        false
    }
}
