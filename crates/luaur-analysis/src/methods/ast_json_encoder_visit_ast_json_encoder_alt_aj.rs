use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_function::AstStatFunction;

impl AstJsonEncoder {
    pub fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
        self.write_ast_stat_function(node);
        false
    }
}
