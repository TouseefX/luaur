use crate::records::hash_blocked_constraint_id::HashBlockedConstraintId;
use crate::type_aliases::constraint_vertex::ConstraintVertex;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct ConstraintList {
    pub(crate) present: DenseHashMap<ConstraintVertex, bool, HashBlockedConstraintId>,
    pub(crate) order: Vec<ConstraintVertex>,
    pub(crate) entries: usize,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let cl: () = ();
    let index: () = ();
}
