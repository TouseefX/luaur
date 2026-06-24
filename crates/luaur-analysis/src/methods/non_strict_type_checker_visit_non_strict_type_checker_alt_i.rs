use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_stat_expr::AstStatExpr;

impl NonStrictTypeChecker {
    pub fn visit_ast_stat_expr(&mut self, expr: *mut AstStatExpr) -> NonStrictContext {
        unsafe { self.visit_ast_expr_value_context((*expr).expr, ValueContext::RValue) }
    }
}
