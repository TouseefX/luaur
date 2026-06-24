//! Source: `Analysis/src/AstJsonEncoder.cpp:1387-1391` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_for::AstStatFor;

impl AstJsonEncoder {
    pub fn visit_ast_stat_for(&mut self, node: *mut AstStatFor) -> bool {
        self.write_ast_stat_for(node);
        false
    }
}
