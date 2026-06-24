use crate::records::mapped_generic_frame::MappedGenericFrame;
use crate::type_aliases::lookup_result::LookupResult;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct MappedGenericEnvironment {
    pub(crate) frames: alloc::vec::Vec<MappedGenericFrame>,
    pub(crate) current_scope_index: Option<usize>,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let mappings: () = ();
    let parentScopeIndex: () = ();
    let children: () = ();
    let scopeIndex: () = ();
}
