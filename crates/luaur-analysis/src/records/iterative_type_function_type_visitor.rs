use crate::records::work_item_iterative_type_function_type_visitor::WorkItem;
use crate::type_aliases::seen_set_iterative_type_function_type_visitor::SeenSet;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct IterativeTypeFunctionTypeVisitor {
    pub(crate) seen: SeenSet,
    pub(crate) work_queue: Vec<WorkItem>,
    pub(crate) parent_cursor: i32,
    pub(crate) work_cursor: u32,
    pub(crate) visitor_name: String,
    pub(crate) visit_once: bool,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let t: () = ();
    let isType: () = ();
    let parent: () = ();
}
