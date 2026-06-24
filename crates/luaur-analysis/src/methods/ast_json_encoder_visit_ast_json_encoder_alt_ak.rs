//! Source: `Analysis/src/AstJsonEncoder.cpp:1417-1421` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl AstJsonEncoder {
    pub fn visit_ast_stat_local_function(&mut self, node: *mut AstStatLocalFunction) -> bool {
        self.write_ast_stat_local_function(node);
        false
    }
}
