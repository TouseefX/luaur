use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::unifier_2::Unifier2;
use core::ptr::NonNull;
use luaur_common::DFInt;

impl Unifier2 {
    pub fn unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
        arena: NonNull<TypeArena>,
        builtin_types: NonNull<BuiltinTypes>,
        scope: NonNull<Scope>,
        ice: NonNull<InternalErrorReporter>,
    ) -> Self {
        Self::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter_dense_hash_set_void(
            arena,
            builtin_types,
            scope,
            ice,
            core::ptr::null_mut(),
        )
    }
}
