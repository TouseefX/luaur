//! Source: `Analysis/src/AstJsonEncoder.cpp:1429-1433` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction;

impl AstJsonEncoder {
    pub fn visit_ast_stat_declare_function(&mut self, node: *mut AstStatDeclareFunction) -> bool {
        self.write_ast_stat_declare_function(node);
        false
    }
}
