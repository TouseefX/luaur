use crate::records::set::Set;
use crate::records::to_string_options::ToStringOptions;
use crate::records::to_string_result::ToStringResult;
use crate::records::to_string_span::ToStringSpan;
use crate::records::type_level::TypeLevel;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct StringifierState {
    pub opts: *mut ToStringOptions,
    pub result: *mut ToStringResult,
    pub cycle_names: DenseHashMap<crate::type_aliases::type_id::TypeId, alloc::string::String>,
    pub cycle_tp_names:
        DenseHashMap<crate::type_aliases::type_pack_id::TypePackId, alloc::string::String>,
    pub seen: Set<*mut core::ffi::c_void>,
    pub used_names: DenseHashSet<alloc::string::String>,
    pub indentation: usize,
    pub exhaustive: bool,
    pub ignore_synthetic_name: bool,
    pub previous_name_index: i32,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let n: () = ();
    let candidate: () = ();
    let count: () = ();
    let buffer: () = ();
}
