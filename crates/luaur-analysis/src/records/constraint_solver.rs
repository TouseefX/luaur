//! Source: `Analysis/include/Luau/ConstraintSolver.h` (hand-ported; fields only)

use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint::Constraint;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::constraint_set::ConstraintSet;
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::dcr_logger::DcrLogger;
use crate::records::hash_blocked_constraint_id::HashBlockedConstraintId;
use crate::records::hash_instantiation_signature::HashInstantiationSignature;
use crate::records::hash_subtype_constraint_record::HashSubtypeConstraintRecord;
use crate::records::instantiation_signature::InstantiationSignature;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module_resolver::ModuleResolver;
use crate::records::normalizer::Normalizer;
use crate::records::require_cycle::RequireCycle;
use crate::records::scope::Scope;
use crate::records::subtype_constraint_record::SubtypeConstraintRecord;
use crate::records::subtyping::Subtyping;
use crate::records::to_string_options::ToStringOptions;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::blocked_constraint_id::BlockedConstraintId;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::boxed::Box;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ConstraintSolver {
    pub arena: *mut TypeArena,
    pub builtin_types: *mut BuiltinTypes,
    pub ice_reporter: InternalErrorReporter,
    pub normalizer: *mut Normalizer,
    pub type_function_runtime: *mut TypeFunctionRuntime,
    pub constraint_set: ConstraintSet,
    pub constraints: Vec<*mut Constraint>,
    pub scope_to_function: *mut DenseHashMap<*mut Scope, TypeId>,
    pub root_scope: *mut Scope,
    pub module: Option<ModulePtr>,
    pub dfg: *const DataFlowGraph,

    pub solver_constraints: Vec<Box<Constraint>>,
    pub solver_constraint_limit: usize,

    pub unsolved_constraints: Vec<*const Constraint>,

    pub deprecated_blocked_constraints: HashMap<*const Constraint, usize>,
    pub deprecated_blocked: HashMap<BlockedConstraintId, DenseHashSet<*const Constraint>>,

    pub instantiated_aliases:
        DenseHashMap<InstantiationSignature, TypeId, HashInstantiationSignature>,
    pub upper_bound_contributors: DenseHashMap<TypeId, Vec<(Location, TypeId)>>,

    pub deprecated_type_to_constraint_set: HashMap<TypeId, HashSet<*const Constraint>>,
    pub deprecated_constraint_to_mutated_types: DenseHashMap<*const Constraint, TypeIds>,

    pub uninhabited_type_functions: DenseHashSet<*const core::ffi::c_void>,
    pub seen_constraints:
        DenseHashMap<SubtypeConstraintRecord, *mut Constraint, HashSubtypeConstraintRecord>,

    pub generalized_types_: DenseHashSet<TypeId>,
    pub generalized_types: *const DenseHashSet<TypeId>,

    pub errors: ErrorVec,
    pub module_resolver: *mut ModuleResolver,
    pub require_cycles: Vec<RequireCycle>,
    pub logger: *mut DcrLogger,
    pub limits: TypeCheckLimits,
    pub type_functions_to_finalize: DenseHashMap<TypeId, *const Constraint>,

    pub opts: ToStringOptions,
    pub cgraph: *mut ConstraintGraph,
    pub subtyping: *mut Subtyping,
}
