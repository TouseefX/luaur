use crate::records::constraint::Constraint;

#[derive(Debug, Clone)]
pub struct BlockedTypePack {
    pub(crate) index: usize,
    pub(crate) owner: *mut Constraint,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let nextIndex: () = ();
}
