use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_pair_hash::TypePairHash;
use crate::records::unifier_2::Unifier2;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::NonNull;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::DFInt;

impl Unifier2 {
    pub fn unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter_dense_hash_set_void(
        _arena: NonNull<TypeArena>,
        _builtin_types: NonNull<BuiltinTypes>,
        _scope: NonNull<Scope>,
        _ice: NonNull<InternalErrorReporter>,
        _uninhabited_type_functions: *mut DenseHashSet<*const core::ffi::c_void>,
    ) -> Self {
        Unifier2 {
            arena: _arena,
            builtin_types: _builtin_types,
            scope: _scope,
            ice: _ice,
            limits: TypeCheckLimits::default(),
            seen_type_pairings: DenseHashSet::<(TypeId, TypeId), TypePairHash>::new((
                core::ptr::null(),
                core::ptr::null(),
            )),
            seen_type_pack_pairings: DenseHashSet::<(TypePackId, TypePackId), TypePairHash>::new((
                core::ptr::null(),
                core::ptr::null(),
            )),
            expanded_free_types: DenseHashMap::new(core::ptr::null()),
            generic_substitutions: DenseHashMap::new(core::ptr::null()),
            generic_pack_substitutions: DenseHashMap::new(core::ptr::null()),
            new_fresh_types: Vec::new(),
            new_fresh_type_packs: Vec::new(),
            iteration_count: 0,
            recursion_count: 0,
            recursion_limit: DFInt::LuauUnifierRecursionLimit.get() as i32,
            incomplete_subtypes: Vec::new(),
            uninhabited_type_functions: _uninhabited_type_functions,
        }
    }
}
