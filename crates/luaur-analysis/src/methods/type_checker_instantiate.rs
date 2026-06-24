use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::function_type::FunctionType;
use crate::records::txn_log::TxnLog;
use crate::records::type_checker::TypeChecker;
use crate::records::unification_too_complex::UnificationTooComplex;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn instantiate(
        &mut self,
        scope: &ScopePtr,
        ty: TypeId,
        location: Location,
        log: *const TxnLog,
    ) -> TypeId {
        let ty = unsafe { follow_type_id(ty) };

        let ftv = unsafe { get_type_id::<FunctionType>(ty) };
        if !ftv.is_null() && unsafe { (*ftv).has_no_free_or_generic_types } {
            return ty;
        }

        // reusableInstantiation.resetState(log, &currentModule->internalTypes, builtinTypes, scope->level, /*scope*/ nullptr);
        unsafe {
            let arena = &mut (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module))
                .internal_types
                as *mut crate::records::type_arena::TypeArena;
            self.reusable_instantiation.reset_state(
                log,
                arena,
                self.builtin_types,
                scope.level.clone(),
                core::ptr::null_mut(),
            );
        }

        if let Some(child_limit) = self.instantiation_child_limit {
            self.reusable_instantiation.base.base.child_limit = child_limit;
        }

        let instantiated = self.reusable_instantiation.substitute_type_id(ty);

        if let Some(instantiated) = instantiated {
            instantiated
        } else {
            self.report_error_location_type_error_data(
                &location,
                UnificationTooComplex::default().into(),
            );
            self.error_recovery_type_scope_ptr(scope)
        }
    }
}
