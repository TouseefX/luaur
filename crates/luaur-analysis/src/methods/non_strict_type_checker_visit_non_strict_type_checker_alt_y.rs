use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_group::AstExprGroup;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_group_value_context(
        &mut self,
        group: *mut AstExprGroup,
        context: ValueContext,
    ) -> NonStrictContext {
        unsafe { self.visit_ast_expr_value_context((*group).expr, context) }
    }
}
