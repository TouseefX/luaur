use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_error::AstExprError;

impl TypeChecker2 {
    pub fn visit_ast_expr_error(&mut self, expr: *mut AstExprError) {
        let expr_ref = unsafe { &*expr };
        let expressions = expr_ref.expressions;
        for i in 0..expressions.size {
            let e = unsafe { *expressions.data.add(i) };
            self.visit_ast_expr_value_context(e, ValueContext::RValue);
        }
    }
}
