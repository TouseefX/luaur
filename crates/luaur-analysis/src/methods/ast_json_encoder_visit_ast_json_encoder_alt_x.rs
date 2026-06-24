use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_if::AstStatIf;

impl AstJsonEncoder {
    pub fn visit_ast_stat_if(&mut self, node: *mut AstStatIf) -> bool {
        unsafe {
            self.write_ast_stat_if(node);
        }
        false
    }
}
