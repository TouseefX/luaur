use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_index_name_value_context(
        &mut self,
        index_name: *mut AstExprIndexName,
        context: ValueContext,
    ) -> NonStrictContext {
        unsafe {
            let expr = (*index_name).expr;
            self.visit_ast_expr_value_context(expr, context)
        }
    }
}
