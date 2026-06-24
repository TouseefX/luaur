use crate::functions::try_get_l_value::try_get_l_value;
use crate::records::truthy_predicate::TruthyPredicate;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::records::unknown_symbol::Context;
use crate::records::unknown_symbol::UnknownSymbol;
use crate::records::with_predicate::WithPredicate;
use crate::type_aliases::l_value::LValue;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeChecker {
    pub fn check_expr_scope_ptr_ast_expr_global(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprGlobal,
    ) -> WithPredicate<TypeId> {
        let lvalue = try_get_l_value(&expr.base);
        LUAU_ASSERT!(lvalue.is_some());

        if let Some(ty) =
            self.resolve_l_value_scope_ptr_l_value(scope.clone(), &lvalue.clone().unwrap())
        {
            let predicate = TruthyPredicate {
                lvalue: lvalue.unwrap(),
                location: expr.base.base.location,
            };
            return WithPredicate::with_predicate_t_predicate_vec(
                ty,
                crate::type_aliases::predicate_vec::PredicateVec::from(vec![
                    crate::type_aliases::predicate::Predicate::Truthy(predicate),
                ]),
            );
        }

        let name_str = unsafe { core::ffi::CStr::from_ptr(expr.name.value).to_string_lossy() };
        let error_data = TypeErrorData::UnknownSymbol(UnknownSymbol::new(
            name_str.to_string(),
            Context::Binding,
        ));
        let error =
            TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
        self.report_error_type_error(&error);

        WithPredicate::with_predicate_t(self.error_recovery_type_scope_ptr(scope))
    }
}
