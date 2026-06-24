use crate::enums::type_context::TypeContext;
use crate::enums::value_context::ValueContext;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_if_else::AstExprIfElse;

impl TypeChecker2 {
    pub fn visit_ast_expr_if_else(&mut self, expr: *mut AstExprIfElse) {
        let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);
        {
            let _in_context_cond =
                InConditionalContext::new(&mut self.type_context, TypeContext::Condition);
            unsafe {
                self.visit_ast_expr_value_context((*expr).condition, ValueContext::RValue);
            }
        }
        unsafe {
            self.visit_ast_expr_value_context((*expr).true_expr, ValueContext::RValue);
            self.visit_ast_expr_value_context((*expr).false_expr, ValueContext::RValue);
        }
    }
}
