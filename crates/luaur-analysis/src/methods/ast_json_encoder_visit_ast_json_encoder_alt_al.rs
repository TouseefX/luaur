//! Source: `Analysis/src/AstJsonEncoder.cpp:1423-1427` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;

impl AstJsonEncoder {
    pub fn visit_ast_stat_type_alias(&mut self, node: *mut AstStatTypeAlias) -> bool {
        self.write_ast_stat_type_alias(node);
        false
    }
}
