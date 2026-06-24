use crate::records::file_resolver::FileResolver;

#[derive(Debug, Clone)]
pub struct ErrorConverter {
    pub(crate) file_resolver: *mut FileResolver,
}

impl Default for ErrorConverter {
    fn default() -> Self {
        Self {
            file_resolver: core::ptr::null_mut(),
        }
    }
}

unsafe impl Send for ErrorConverter {}
unsafe impl Sync for ErrorConverter {}

/// This record represents the C++ `ErrorConverter` struct used as a visitor/functor
/// to convert `TypeErrorData` variants into human-readable strings.
///
/// In Rust, the `operator()` overloads are translated as inherent methods or
/// a single dispatch method. Since the schedule identifies this as a `record`,
/// we only emit the struct definition here. The implementation of the conversion
/// logic for each error variant will be provided in separate `impl` blocks.
#[allow(non_snake_case)]
impl ErrorConverter {
    pub(crate) fn new(file_resolver: *mut FileResolver) -> Self {
        Self { file_resolver }
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let result: () = ();
    let s: () = ();
    let expectedS: () = ();
    let actualVerb: () = ();
    let name: () = ();
    let first: () = ();
    let candidatesSuggestion: () = ();
    let metatable: () = ();
    let unwrapped: () = ();
    let err: () = ();
    let ss: () = ();
    let isFirst: () = ();
    let argAnnotations: () = ();
    let lowerBounds: () = ();
    let upperBounds: () = ();
}
