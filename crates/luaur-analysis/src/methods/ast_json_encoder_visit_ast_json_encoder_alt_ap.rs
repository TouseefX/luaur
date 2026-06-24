//! Source: `Analysis/src/AstJsonEncoder.cpp:1447-1451` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_error::AstStatError;

impl AstJsonEncoder {
    pub fn visit_ast_stat_error(&mut self, node: *mut AstStatError) -> bool {
        self.write_ast_stat_error(node);
        false
    }
}
