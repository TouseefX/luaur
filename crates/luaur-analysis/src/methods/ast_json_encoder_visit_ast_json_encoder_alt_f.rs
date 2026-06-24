//! Source: `Analysis/src/AstJsonEncoder.cpp:1225-1229` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_constant_bool::AstExprConstantBool;

impl AstJsonEncoder {
    pub fn visit_ast_expr_constant_bool(&mut self, node: *mut AstExprConstantBool) -> bool {
        self.write_ast_expr_constant_bool(node);
        false
    }
}
