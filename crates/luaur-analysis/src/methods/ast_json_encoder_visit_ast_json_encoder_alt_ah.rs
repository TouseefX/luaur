//! Source: `Analysis/src/AstJsonEncoder.cpp:1399-1403` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_assign::AstStatAssign;

impl AstJsonEncoder {
    pub fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
        self.write_ast_stat_assign(node);
        false
    }
}
