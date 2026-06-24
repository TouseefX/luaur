//! @interface-stub
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::unifier_counters::UnifierCounters;
use crate::records::unifier_shared_state::UnifierSharedState;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl UnifierSharedState {
    pub fn unifier_shared_state(ice_handler: *mut InternalErrorReporter) -> Self {
        Self {
            ice_handler,
            skip_cache_for_type: DenseHashMap::new(core::ptr::null()),
            cached_unify: DenseHashSet::new((core::ptr::null(), core::ptr::null())),
            cached_unify_error: DenseHashMap::new((core::ptr::null(), core::ptr::null())),
            temp_seen_ty: DenseHashSet::new(core::ptr::null()),
            temp_seen_tp: DenseHashSet::new(core::ptr::null()),
            counters: UnifierCounters::default(),
            reentrant_type_reduction: false,
        }
    }
}
