use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_interp_string(
        &mut self,
        interp_string: *mut AstExprInterpString,
    ) -> NonStrictContext {
        let expressions = unsafe { (*interp_string).expressions };
        for i in 0..expressions.size {
            let expr = unsafe { *expressions.data.add(i) };
            self.visit_ast_expr_value_context(expr, ValueContext::RValue);
        }

        NonStrictContext::non_strict_context()
    }
}
