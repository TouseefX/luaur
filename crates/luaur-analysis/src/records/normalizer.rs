use crate::enums::solver_mode::SolverMode;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::normalized_type::NormalizedType;
use crate::records::type_arena::TypeArena;
use crate::records::type_id_pair_hash::TypeIdPairHash;
use crate::records::type_ids::TypeIds;
use crate::records::unifier_shared_state::UnifierSharedState;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::sync::Arc;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct Normalizer {
    pub(crate) cached_normals: alloc::collections::BTreeMap<TypeId, Arc<NormalizedType>>,
    pub(crate) cached_intersections: alloc::collections::BTreeMap<*const TypeIds, TypeId>,
    pub(crate) cached_unions: alloc::collections::BTreeMap<*const TypeIds, TypeId>,
    pub(crate) cached_type_ids: alloc::collections::BTreeMap<*const TypeIds, Box<TypeIds>>,
    pub(crate) cached_is_inhabited: DenseHashMap<TypeId, bool>,
    pub(crate) cached_is_inhabited_intersection:
        DenseHashMap<(TypeId, TypeId), bool, TypeIdPairHash>,
    pub(crate) fuel: Option<i32>,
    pub(crate) arena: *mut TypeArena,
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) shared_state: *mut UnifierSharedState,
    pub(crate) cache_inhabitance: bool,
    pub(crate) solver_mode: SolverMode,
}
