use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_stat_local_function::AstStatLocalFunction;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_local_function(
        &mut self,
        local_fn: *mut AstStatLocalFunction,
    ) -> NonStrictContext {
        unsafe {
            // C++ `visit(localFn->func, ValueContext::RValue)` dispatches via the
            // generic `visit(AstExpr*, ValueContext)` overload (AstExprFunction* upcasts).
            let func = (*local_fn).func as *mut luaur_ast::records::ast_expr::AstExpr;
            self.visit_ast_expr_value_context(func, ValueContext::RValue)
        }
    }
}
