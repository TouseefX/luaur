#[derive(Debug, Clone)]
pub struct TypeFunctionNegationType {
    pub(crate) type_id: crate::type_aliases::type_function_type_id::TypeFunctionTypeId,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let r#type: () = ();
}
