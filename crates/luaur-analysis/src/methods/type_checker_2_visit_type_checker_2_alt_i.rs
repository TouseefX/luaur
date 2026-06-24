use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_expr::AstStatExpr;

impl TypeChecker2 {
    pub fn visit_ast_stat_expr(&mut self, expr: *mut AstStatExpr) {
        unsafe {
            self.visit_ast_expr_value_context((*expr).expr, ValueContext::RValue);
        }
    }
}
