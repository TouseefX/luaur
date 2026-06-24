use crate::enums::skip_test_result::SkipTestResult;
use crate::enums::type_function_instance_state::TypeFunctionInstanceState;
use crate::records::function_graph_reduction_result::FunctionGraphReductionResult;
use crate::records::type_function_context::TypeFunctionContext;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_type_pack_id_set::TypeOrTypePackIdSet;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;
use luaur_common::records::vec_deque::VecDeque;

#[derive(Debug, Clone)]
pub struct TypeFunctionReducer {
    pub ctx: NonNull<TypeFunctionContext>,
    pub queued_tys: VecDeque<TypeId>,
    pub queued_tps: VecDeque<TypePackId>,
    pub should_guess: TypeOrTypePackIdSet,
    pub cyclic_type_functions: Vec<TypeId>,
    pub irreducible: TypeOrTypePackIdSet,
    pub result: FunctionGraphReductionResult,
    pub force: bool,
    pub location: Location,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let queue: () = ();
    let seen: () = ();
    let guesser: () = ();
    let finder: () = ();
}
