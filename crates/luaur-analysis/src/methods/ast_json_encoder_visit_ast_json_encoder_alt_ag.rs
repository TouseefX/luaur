//! Source: `Analysis/src/AstJsonEncoder.cpp:1393-1397` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;

impl AstJsonEncoder {
    pub fn visit_ast_stat_for_in(&mut self, node: *mut AstStatForIn) -> bool {
        self.write_ast_stat_for_in(node);
        false
    }
}
