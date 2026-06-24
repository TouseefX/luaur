use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_while::AstStatWhile;

impl AstJsonEncoder {
    pub fn visit_ast_stat_while(&mut self, node: *mut AstStatWhile) -> bool {
        unsafe {
            self.write_ast_stat_while(node);
        }
        false
    }
}
