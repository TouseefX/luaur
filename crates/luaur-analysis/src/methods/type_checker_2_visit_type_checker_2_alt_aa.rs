use crate::enums::value_context::ValueContext;
use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_expr_group::AstExprGroup;

impl TypeChecker2 {
    pub fn visit_ast_expr_group_value_context(
        &mut self,
        expr: *mut AstExprGroup,
        context: ValueContext,
    ) {
        unsafe {
            self.visit_ast_expr_value_context((*expr).expr, context);
        }
    }
}
