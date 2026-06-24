use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_index_expr_value_context(
        &mut self,
        index_expr: *mut AstExprIndexExpr,
        context: ValueContext,
    ) -> NonStrictContext {
        let expr = unsafe { (*index_expr).expr };
        let index = unsafe { (*index_expr).index };

        let expr_context = self.visit_ast_expr_value_context(expr, context);
        let index_context = self.visit_ast_expr_value_context(index, ValueContext::RValue);

        NonStrictContext::disjunction(
            self.builtin_types,
            self.arena,
            &expr_context,
            &index_context,
        )
    }
}
