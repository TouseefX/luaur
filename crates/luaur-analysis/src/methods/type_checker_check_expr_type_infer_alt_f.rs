use crate::functions::try_get_l_value::try_get_l_value;
use crate::records::truthy_predicate::TruthyPredicate;
use crate::records::type_checker::TypeChecker;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::predicate::Predicate;
use crate::type_aliases::predicate_vec::PredicateVec;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl TypeChecker {
    pub fn check_expr_scope_ptr_ast_expr_index_name(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprIndexName,
    ) -> WithPredicate<TypeId> {
        let name: crate::type_aliases::name_type_infer::Name = unsafe {
            core::ffi::CStr::from_ptr(expr.index.value)
                .to_string_lossy()
                .into_owned()
        };

        // Redundant call if we find a refined lvalue, but this function must be called in order to recursively populate astTypes.
        let mut lhs_type = self
            .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
                scope,
                unsafe { &*expr.expr },
                None,
                false,
            )
            .r#type;

        if let Some(lvalue) = try_get_l_value(&expr.base) {
            if let Some(ty) = self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue) {
                let predicate = TruthyPredicate {
                    lvalue,
                    location: expr.base.base.location,
                };
                return WithPredicate::with_predicate_t_predicate_vec(
                    ty,
                    PredicateVec::from(vec![Predicate::Truthy(predicate)]),
                );
            }
        }

        lhs_type = self.strip_from_nil_and_report(lhs_type, unsafe { &(*expr.expr).base.location });

        if let Some(ty) = self.get_index_type_from_type(
            scope.clone(),
            lhs_type,
            &name,
            &expr.base.base.location,
            true,
        ) {
            return WithPredicate::with_predicate_t(ty);
        }

        WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
    }
}
