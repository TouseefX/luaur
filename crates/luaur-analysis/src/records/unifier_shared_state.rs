use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::type_id_pair_hash::TypeIdPairHash;
use crate::records::unifier_counters::UnifierCounters;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug)]
pub struct UnifierSharedState {
    pub(crate) ice_handler: *mut InternalErrorReporter,
    pub(crate) skip_cache_for_type: DenseHashMap<TypeId, bool>,
    pub(crate) cached_unify: DenseHashSet<(TypeId, TypeId), TypeIdPairHash>,
    pub(crate) cached_unify_error: DenseHashMap<(TypeId, TypeId), TypeErrorData, TypeIdPairHash>,
    pub(crate) temp_seen_ty: DenseHashSet<TypeId>,
    pub(crate) temp_seen_tp: DenseHashSet<TypePackId>,
    pub(crate) counters: UnifierCounters,
    pub(crate) reentrant_type_reduction: bool,
}
