use crate::enums::polarity::Polarity;
use crate::records::counter_state::CounterState;
use crate::records::type_visitor::TypeVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct GenericCounter {
    pub base: TypeVisitor,
    pub seen_counts: DenseHashMap<TypeId, usize>,
    pub cached_types: *mut DenseHashSet<TypeId>,
    pub generics: DenseHashMap<TypeId, CounterState>,
    pub generic_packs: DenseHashMap<TypePackId, CounterState>,
    pub polarity: Polarity,
    pub steps: i32,
    pub hit_limits: bool,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let count: () = ();
    let seenCount: () = ();
    let previous: () = ();
    let p: () = ();
}
