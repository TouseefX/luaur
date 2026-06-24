use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_instantiate::AstExprInstantiate;

impl TypeChecker {
    pub fn check_expr_scope_ptr_ast_expr_instantiate(
        &mut self,
        scope: &ScopePtr,
        explicit_type_instantiation: &AstExprInstantiate,
    ) -> WithPredicate<TypeId> {
        if !crate::FFlag::LuauExplicitTypeInstantiationSupport.get() {
            return WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope));
        }

        let base_expr = unsafe { &*explicit_type_instantiation.expr };
        let base_type =
            self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(scope, base_expr, None, false);

        WithPredicate::with_predicate_t(self.instantiate_type_parameters(
            scope.clone(),
            base_type.r#type,
            explicit_type_instantiation.type_arguments,
            base_expr as *const _,
            &base_expr.base.location,
        ))
    }
}
