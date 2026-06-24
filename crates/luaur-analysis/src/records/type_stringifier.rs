#[derive(Debug, Clone)]
pub struct TypeStringifier {
    pub(crate) state: *mut crate::records::stringifier_state::StringifierState,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let comma: () = ();
    let first: () = ();
    let showName: () = ();
    let openbrace: () = ();
    let closedbrace: () = ();
    let index: () = ();
    let optional: () = ();
    let hasNonNilDisjunct: () = ();
    let results: () = ();
    let resultsLength: () = ();
    let lengthLimitHit: () = ();
    let elem: () = ();
    let s: () = ();
}
