use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_error::AstExprError;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_error(&mut self, error: *mut AstExprError) -> NonStrictContext {
        unsafe {
            let error_ref = &*error;
            for i in 0..error_ref.expressions.size {
                let expr = *error_ref.expressions.data.add(i);
                self.visit_ast_expr_value_context(expr, ValueContext::RValue);
            }
        }
        NonStrictContext::non_strict_context()
    }
}
