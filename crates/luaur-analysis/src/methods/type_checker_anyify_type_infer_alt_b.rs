use crate::records::anyification::Anyification;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::type_checker::TypeChecker;
use crate::records::unification_too_complex::UnificationTooComplex;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn anyify_scope_ptr_type_pack_id_location(
        &mut self,
        scope: ScopePtr,
        ty: TypePackId,
        location: Location,
    ) -> TypePackId {
        let arena = unsafe {
            &mut (*(alloc::sync::Arc::as_ptr(self.current_module.as_ref().unwrap())
                as *mut crate::records::module::Module))
                .internal_types as *mut crate::records::type_arena::TypeArena
        };
        let mut anyification = Anyification::anyification_type_arena_scope_ptr_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
            arena,
            &scope,
            self.builtin_types,
            self.ice_handler,
            self.any_type,
            self.any_type_pack,
        );
        let any = anyification.base.substitute_type_pack_id(ty);
        if let Some(any) = any {
            any
        } else {
            self.report_error_location_type_error_data(
                &location,
                UnificationTooComplex::default().into(),
            );
            self.error_recovery_type_pack_type_pack_id(self.any_type_pack)
        }
    }
}
