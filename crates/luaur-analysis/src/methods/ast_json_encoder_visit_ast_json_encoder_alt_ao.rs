//! Source: `Analysis/src/AstJsonEncoder.cpp:1441-1445` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

impl AstJsonEncoder {
    pub fn visit_ast_stat_declare_extern_type(
        &mut self,
        node: *mut AstStatDeclareExternType,
    ) -> bool {
        self.write_ast_stat_declare_extern_type(node);
        false
    }
}
