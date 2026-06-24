use crate::records::anyification::Anyification;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::scope_ptr_anyification::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl Anyification {
    pub fn anyification_type_arena_scope_ptr_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
        arena: *mut TypeArena,
        scope: &ScopePtr,
        builtin_types: *mut BuiltinTypes,
        ice_handler: *mut InternalErrorReporter,
        any_type: TypeId,
        any_type_pack: TypePackId,
    ) -> Self {
        let scope_raw = alloc::sync::Arc::as_ptr(scope) as *mut Scope;
        Self::anyification_type_arena_not_null_scope_not_null_builtin_types_internal_error_reporter_type_id_type_pack_id(
            arena,
            scope_raw,
            builtin_types,
            ice_handler,
            any_type,
            any_type_pack,
        )
    }
}
