//! Source: `Analysis/src/AstJsonEncoder.cpp:1381-1385` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl AstJsonEncoder {
    pub fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
        self.write_ast_stat_local(node);
        false
    }
}
