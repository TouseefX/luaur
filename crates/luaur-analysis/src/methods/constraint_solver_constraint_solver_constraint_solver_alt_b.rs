use crate::functions::borrow_constraints::borrow_constraints;
use crate::records::constraint::Constraint;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::constraint_set::ConstraintSet;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::dcr_logger::DcrLogger;
use crate::records::module_resolver::ModuleResolver;
use crate::records::normalizer::Normalizer;
use crate::records::require_cycle::RequireCycle;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::module_ptr_module::ModulePtr;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::FInt;

impl ConstraintSolver {
    pub fn constraint_solver_not_null_normalizer_not_null_type_function_runtime_not_null_scope_vector_not_null_constraint_not_null_dense_hash_map_scope_type_id_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
        normalizer: *const Normalizer,
        type_function_runtime: *const TypeFunctionRuntime,
        root_scope: NonNull<Scope>,
        constraints: alloc::vec::Vec<NonNull<Constraint>>,
        scope_to_function: NonNull<DenseHashMap<*mut Scope, crate::type_aliases::type_id::TypeId>>,
        module: ModulePtr,
        module_resolver: *const ModuleResolver,
        require_cycles: alloc::vec::Vec<RequireCycle>,
        logger: *mut DcrLogger,
        dfg: *const DataFlowGraph,
        limits: TypeCheckLimits,
        cgraph: *mut ConstraintGraph,
        subtyping: *const Subtyping,
    ) -> Self {
        // C++ `constraintSet{rootScope}` aggregate-initializes only `rootScope`;
        // every other field is default-constructed.
        let mut constraint_set = ConstraintSet {
            root_scope: root_scope.as_ptr(),
            constraints: Vec::new(),
            free_types: crate::records::type_ids::TypeIds::type_ids(),
            scope_to_function: DenseHashMap::new(core::ptr::null_mut()),
            errors: Vec::new(),
        };
        constraint_set.constraints = constraints
            .iter()
            .map(|c| unsafe { c.as_ptr() })
            .collect::<Vec<_>>();
        constraint_set.root_scope = root_scope.as_ptr();

        let mut result = ConstraintSolver::constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
            normalizer,
            type_function_runtime,
            module,
            module_resolver,
            require_cycles,
            logger,
            dfg,
            limits.clone(),
            constraint_set,
            cgraph,
            subtyping,
        );

        // The delegated base constructor already populated `result.constraints`
        // (from `constraint_set.constraints`, which holds the same pointers as the
        // `constraints` argument) and already ran `init_free_type_tracking()`.
        // Re-running it here would double-insert every constraint and trip the
        // `LUAU_ASSERT(fresh1)` in `deprecated_constraint_to_mutated_types`. We only
        // need to point the solver at the fragment generator's `scopeToFunction` and
        // `rootScope` (C++ `NotNull{&cg.scopeToFunction}`, `NotNull(cg.rootScope)`).
        result.scope_to_function = scope_to_function.as_ptr();
        result.root_scope = root_scope.as_ptr();
        result.solver_constraint_limit = FInt::LuauSolverConstraintLimit.get() as usize;
        result.module_resolver = module_resolver as *mut ModuleResolver;
        result.logger = logger;
        result.limits = limits;
        result.cgraph = cgraph;
        result.subtyping = subtyping as *mut Subtyping;

        result
    }
}
