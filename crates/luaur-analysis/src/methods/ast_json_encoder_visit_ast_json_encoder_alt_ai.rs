//! Source: `Analysis/src/AstJsonEncoder.cpp:1405-1409` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;

impl AstJsonEncoder {
    pub fn visit_ast_stat_compound_assign(&mut self, node: *mut AstStatCompoundAssign) -> bool {
        self.write_ast_stat_compound_assign(node);
        false
    }
}
