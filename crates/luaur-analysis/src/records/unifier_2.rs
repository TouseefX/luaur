use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::NonNull;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug)]
pub struct Unifier2 {
    pub(crate) arena: NonNull<TypeArena>,
    pub(crate) builtin_types: NonNull<BuiltinTypes>,
    pub(crate) scope: NonNull<Scope>,
    pub(crate) ice: NonNull<InternalErrorReporter>,
    pub(crate) limits: TypeCheckLimits,
    pub(crate) seen_type_pairings: DenseHashSet<(TypeId, TypeId), TypePairHash>,
    pub(crate) seen_type_pack_pairings: DenseHashSet<(TypePackId, TypePackId), TypePairHash>,
    pub(crate) expanded_free_types: DenseHashMap<TypeId, Vec<TypeId>>,
    pub(crate) generic_substitutions: DenseHashMap<TypeId, TypeId>,
    pub(crate) generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
    pub(crate) new_fresh_types: Vec<TypeId>,
    pub(crate) new_fresh_type_packs: Vec<TypePackId>,
    pub(crate) iteration_count: i32,
    pub(crate) recursion_count: i32,
    pub(crate) recursion_limit: i32,
    pub(crate) incomplete_subtypes: Vec<ConstraintV>,
    pub(crate) uninhabited_type_functions: *mut DenseHashSet<*const c_void>,
}
