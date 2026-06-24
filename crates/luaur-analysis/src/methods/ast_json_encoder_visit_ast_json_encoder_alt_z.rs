use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;

impl AstJsonEncoder {
    pub fn visit_ast_stat_repeat(&mut self, node: *mut AstStatRepeat) -> bool {
        self.write_ast_stat_repeat(node);
        false
    }
}
