//! Source: `Analysis/src/AstJsonEncoder.cpp:1315-1319` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;

impl AstJsonEncoder {
    pub fn visit_ast_expr_type_assertion(&mut self, node: *mut AstExprTypeAssertion) -> bool {
        self.write_ast_expr_type_assertion(node);
        false
    }
}
