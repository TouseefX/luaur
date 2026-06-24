//! Source: `Analysis/src/AstJsonEncoder.cpp:1243-1247` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;

impl AstJsonEncoder {
    pub fn visit_ast_expr_if_else(&mut self, node: *mut AstExprIfElse) -> bool {
        self.write_ast_expr_if_else(node);
        false
    }
}
