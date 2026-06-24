use crate::enums::type_context::TypeContext;
use crate::enums::value_context::ValueContext;
use crate::records::in_conditional_context::InConditionalContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_table::AstExprTable;

impl TypeChecker2 {
    pub fn visit_ast_expr_table(&mut self, expr: *mut AstExprTable) {
        unsafe {
            let _in_context = InConditionalContext::new(
                &mut self.type_context as *mut TypeContext,
                TypeContext::Default,
            );

            for item in (*expr).items.iter() {
                if !item.key.is_null() {
                    self.visit_ast_expr_value_context(item.key, ValueContext::RValue);
                }
                self.visit_ast_expr_value_context(item.value, ValueContext::RValue);
            }
        }
    }
}
