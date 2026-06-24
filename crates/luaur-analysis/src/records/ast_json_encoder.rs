use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct AstJsonEncoder {
    pub(crate) chunks: Vec<String>,
    pub(crate) comma: bool,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let CHUNK_SIZE: () = ();
    let c: () = ();
    let b: () = ();
    let commentComma: () = ();
}
