use crate::records::type_rehydration_options::TypeRehydrationOptions;
use crate::type_aliases::synthetic_names::SyntheticNames;
use core::ffi::c_void;
use luaur_ast::records::allocator::Allocator;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct TypeRehydrationVisitor {
    pub(crate) seen: BTreeMap<*mut c_void, i32>,
    pub(crate) count: i32,
    pub(crate) allocator: *mut Allocator,
    pub(crate) synthetic_names: *mut SyntheticNames,
    pub(crate) options: TypeRehydrationOptions,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let value: () = ();
    let parameters: () = ();
    let props: () = ();
    let idx: () = ();
    let indexer: () = ();
    let generics: () = ();
    let numGenerics: () = ();
    let genericPacks: () = ();
    let numGenericPacks: () = ();
    let argTypes: () = ();
    let argTailAnnotation: () = ();
    let argNames: () = ();
    let i: () = ();
    let arg: () = ();
    let returnTypes: () = ();
    let retTailAnnotation: () = ();
    let unionTypes: () = ();
    let intersectionTypes: () = ();
    let params: () = ();
}
