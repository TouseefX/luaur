use crate::records::substitution::Substitution;
use crate::records::tarjan::SubstitutionVtable;
use crate::records::tarjan::Tarjan;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl Substitution {
    pub fn substitution_type_arena(&mut self, arena: *mut TypeArena) {
        self.substitution_txn_log_type_arena(TxnLog::empty(), arena);
    }

    /// C++ `Substitution::Substitution(const TxnLog* log_, TypeArena* arena)`
    /// (`Substitution.cpp`). Builds a fresh `Substitution` value with empty
    /// containers and the given log/arena; the C++ base `Tarjan()` constructor
    /// reserves space for its worklists, mirrored by `Tarjan::tarjan`.
    pub fn substitution_new(log_: *const TxnLog, arena: *mut TypeArena) -> Self {
        let mut base = Tarjan {
            type_to_index: DenseHashMap::new(core::ptr::null()),
            pack_to_index: DenseHashMap::new(core::ptr::null()),
            nodes: alloc::vec::Vec::new(),
            stack: alloc::vec::Vec::new(),
            child_count: 0,
            child_limit: 0,
            log: core::ptr::null(),
            edges_ty: alloc::vec::Vec::new(),
            edges_tp: alloc::vec::Vec::new(),
            worklist: alloc::vec::Vec::new(),
            vtable: SubstitutionVtable::null(),
        };
        base.tarjan();

        let mut this = Substitution {
            base,
            arena: core::ptr::null_mut(),
            new_types: DenseHashMap::new(core::ptr::null()),
            new_packs: DenseHashMap::new(core::ptr::null()),
            replaced_types: DenseHashSet::new(core::ptr::null()),
            replaced_type_packs: DenseHashSet::new(core::ptr::null()),
            no_traverse_types: DenseHashSet::new(core::ptr::null()),
            no_traverse_type_packs: DenseHashSet::new(core::ptr::null()),
        };
        this.substitution_txn_log_type_arena(log_, arena);
        this
    }
}
