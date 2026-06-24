use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_return::AstStatReturn;

impl AstJsonEncoder {
    pub fn visit_ast_stat_return(&mut self, node: *mut AstStatReturn) -> bool {
        self.write_ast_stat_return(node);
        false
    }
}
