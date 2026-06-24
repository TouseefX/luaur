use crate::records::type_checker::TypeChecker;
use luaur_ast::records::ast_expr_interp_string::AstExprInterpString;

impl TypeChecker {
    pub fn check_expr_scope_ptr_ast_expr_interp_string(
        &mut self,
        scope: &crate::type_aliases::scope_ptr_type::ScopePtr,
        expr: &AstExprInterpString,
    ) -> crate::records::with_predicate::WithPredicate<crate::type_aliases::type_id::TypeId> {
        for i in 0..expr.expressions.size {
            let child = unsafe { *expr.expressions.data.add(i) };
            self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*child },
                None,
                false,
            );
        }

        crate::records::with_predicate::WithPredicate::with_predicate_t(self.string_type)
    }
}
