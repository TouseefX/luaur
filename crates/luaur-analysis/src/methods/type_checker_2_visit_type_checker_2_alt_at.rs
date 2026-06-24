use crate::enums::type_context::TypeContext;
use crate::enums::value_context::ValueContext;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;

impl TypeChecker2 {
    pub fn visit_ast_expr_interp_string(&mut self, interp_string: *mut AstExprInterpString) {
        let in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

        let expressions = unsafe { (*interp_string).expressions };
        for i in 0..expressions.size {
            let expr = unsafe { *expressions.data.add(i) };
            self.visit_ast_expr_value_context(expr, ValueContext::RValue);
        }

        core::mem::drop(in_context);
    }
}
