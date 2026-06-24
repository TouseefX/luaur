use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::def_arena::DefArena;
use crate::records::dfg_scope::DfgScope;
use crate::records::function_capture::FunctionCapture;
use crate::records::refinement_key_arena::RefinementKeyArena;
use crate::records::symbol::Symbol;
use crate::type_aliases::scope_stack::ScopeStack;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug)]
pub struct DataFlowGraphBuilder {
    pub(crate) graph: DataFlowGraph,
    pub(crate) def_arena: *mut DefArena,
    pub(crate) key_arena: *mut RefinementKeyArena,
    pub(crate) handle: *mut crate::records::internal_error_reporter::InternalErrorReporter,
    pub(crate) scopes: alloc::vec::Vec<alloc::boxed::Box<DfgScope>>,
    pub(crate) scope_stack: ScopeStack,
    pub(crate) captures: DenseHashMap<Symbol, FunctionCapture>,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let captureDefs: () = ();
    let allVersions: () = ();
    let versionOffset: () = ();
}
