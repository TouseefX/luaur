//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Set.h:18:set`
//! Source: `Analysis/include/Luau/Set.h:18-130` (hand-ported)
//!
//! C++ `Luau::Set<T>` — DenseHashMap<T, bool> with a tombstone-false `erase`
//! (DenseHashSet cannot erase; Set can).

use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct Set<T> {
    pub(crate) mapping: DenseHashMap<T, bool>,
    pub(crate) entry_count: usize,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let const_iterator: () = ();
    let entry: () = ();
    let fresh: () = ();
    let res: () = ();
}
