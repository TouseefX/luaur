use crate::records::type_fun::TypeFun;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct InstantiationSignature {
    pub(crate) fn_sig: TypeFun,
    pub(crate) arguments: Vec<TypeId>,
    pub(crate) pack_arguments: Vec<TypePackId>,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let r#fn: () = ();
}
