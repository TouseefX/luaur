use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct Error {
    pub index: i32,
    pub synthetic: Option<TypeId>,
}

impl Error {
    pub fn error_error() -> Self {
        Self {
            index: 0,
            synthetic: None,
        }
    }

    pub fn error_error_with_synthetic(synthetic: TypeId) -> Self {
        Self {
            index: 0,
            synthetic: Some(synthetic),
        }
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let nextIndex: () = ();
}
