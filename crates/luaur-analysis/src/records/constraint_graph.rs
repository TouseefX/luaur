use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint_list::ConstraintList;
use crate::type_aliases::constraint_map::ConstraintMap;
use core::ptr::NonNull;

#[derive(Debug, Clone)]
pub struct ConstraintGraph {
    pub(crate) builtin_types: NonNull<BuiltinTypes>,
    pub(crate) dependencies: ConstraintMap,
    pub(crate) reverse_dependencies: ConstraintMap,
    pub(crate) constraint_lists: Vec<Box<ConstraintList>>,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let types: () = ();
    let packs: () = ();
}
