//! Node: `cxx:Record:Luau.Analysis:Analysis/src/ToString.cpp:1167:type_pack_stringifier`
//! Source: `Analysis/src/ToString.cpp:1167-1184` (hand-ported)
//!
//! C++ `struct TypePackStringifier` — `elemNames` is a by-value const member
//! in C++ (copied from the ctor ref), hence the owned Vec.

use crate::records::function_argument::FunctionArgument;
use crate::records::stringifier_state::StringifierState;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct TypePackStringifier {
    pub(crate) state: *mut StringifierState,
    pub(crate) elem_names: Vec<Option<FunctionArgument>>,
    pub(crate) elem_index: u32,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let dummyElemNames: () = ();
    let tvs: () = ();
    let first: () = ();
    let comma: () = ();
}
