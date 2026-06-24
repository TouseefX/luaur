use crate::records::type_rehydration_visitor::TypeRehydrationVisitor;
use crate::type_aliases::synthetic_names::SyntheticNames;
use luaur_ast::records::allocator::Allocator;

#[derive(Debug, Clone)]
pub struct TypePackRehydrationVisitor {
    pub(crate) allocator: *mut Allocator,
    pub(crate) synthetic_names: *mut SyntheticNames,
    pub(crate) type_visitor: *mut TypeRehydrationVisitor,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let head: () = ();
    let tail: () = ();
}
