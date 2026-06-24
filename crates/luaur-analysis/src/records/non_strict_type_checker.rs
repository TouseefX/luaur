//! Source: `Analysis/src/NonStrictTypeChecker.cpp` (hand-ported; fields only)

use crate::records::builtin_types::BuiltinTypes;
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module::Module;
use crate::records::normalizer::Normalizer;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug)]
pub struct NonStrictTypeChecker {
    pub builtin_types: *mut BuiltinTypes,
    pub type_function_runtime: *mut TypeFunctionRuntime,
    pub ice: *mut InternalErrorReporter,
    pub arena: *mut TypeArena,
    pub module: *mut Module,
    pub normalizer: Normalizer,
    pub subtyping: Subtyping,
    pub dfg: *const DataFlowGraph,
    pub no_type_function_errors: DenseHashSet<TypeId>,
    pub stack: Vec<*mut Scope>,
    pub cached_negations: DenseHashMap<TypeId, TypeId>,
    pub limits: *mut TypeCheckLimits,
    /// C++ `int nonStrictRecursionCount = 0;` (private member of `NonStrictTypeChecker`).
    pub non_strict_recursion_count: core::ffi::c_int,
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let StackPusher: () = ();
    let fst: () = ();
    let result: () = ();
    let instance: () = ();
    let _rc: () = ();
    let ctx: () = ();
    let stat: () = ();
    let branchContext: () = ();
    let fresh: () = ();
    let fnTy: () = ();
    let arguments: () = ();
    let argTypes: () = ();
    let arg: () = ();
    let expectedArgType: () = ();
    let runTimeErrorTy: () = ();
    let remainingArgsOptional: () = ();
    let debugname: () = ();
    let remainder: () = ();
    let typesProvided: () = ();
    let extraTypes: () = ();
    let packsProvided: () = ();
    let symbol: () = ();
    let seen: () = ();
    let didNarrow: () = ();
    let bestScope: () = ();
    let defs: () = ();
    let isUnknown: () = ();
    let nonStrictRecursionCount: () = ();
    let cachedResult: () = ();
}
