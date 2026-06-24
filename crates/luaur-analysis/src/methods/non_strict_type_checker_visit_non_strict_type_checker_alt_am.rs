use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_unary::AstExprUnary;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_unary(&mut self, unary: *mut AstExprUnary) -> NonStrictContext {
        unsafe {
            let expr = (*unary).expr;
            self.visit_ast_expr_value_context(expr, ValueContext::RValue)
        }
    }
}
