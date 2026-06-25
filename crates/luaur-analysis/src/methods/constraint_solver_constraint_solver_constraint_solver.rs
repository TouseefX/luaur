use crate::functions::borrow_constraints::borrow_constraints;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::constraint_set::ConstraintSet;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::dcr_logger::DcrLogger;
use crate::records::module_resolver::ModuleResolver;
use crate::records::normalizer::Normalizer;
use crate::records::require_cycle::RequireCycle;
use crate::records::subtyping::Subtyping;
use crate::records::to_string_options::ToStringOptions;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::module_ptr_module::ModulePtr;
use luaur_common::FInt;

impl ConstraintSolver {
    pub fn constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
        normalizer: *const Normalizer,
        type_function_runtime: *const TypeFunctionRuntime,
        module: ModulePtr,
        module_resolver: *const ModuleResolver,
        require_cycles: alloc::vec::Vec<RequireCycle>,
        logger: *mut DcrLogger,
        dfg: *const DataFlowGraph,
        limits: TypeCheckLimits,
        mut constraint_set: ConstraintSet,
        cgraph: *mut ConstraintGraph,
        subtyping: *const Subtyping,
    ) -> Self {
        let empty_instantiation_signature =
            crate::records::instantiation_signature::InstantiationSignature {
                fn_sig: crate::records::type_fun::TypeFun {
                    type_params: alloc::vec::Vec::new(),
                    type_pack_params: alloc::vec::Vec::new(),
                    r#type: core::ptr::null(),
                    definition_location: None,
                },
                arguments: alloc::vec::Vec::new(),
                pack_arguments: alloc::vec::Vec::new(),
            };
        let empty_subtype_constraint =
            crate::records::subtype_constraint_record::SubtypeConstraintRecord {
                subTy: core::ptr::null(),
                superTy: core::ptr::null(),
                variance: crate::enums::subtyping_variance::SubtypingVariance::Invalid,
            };

        let mut result =
            ConstraintSolver {
                arena: unsafe { (*normalizer).arena },
                builtin_types: unsafe { (*normalizer).builtin_types },
                ice_reporter: unsafe { (*type_function_runtime).ice.clone() },
                normalizer: normalizer as *mut Normalizer,
                type_function_runtime: type_function_runtime as *mut TypeFunctionRuntime,
                constraint_set,
                constraints: alloc::vec::Vec::new(),
                scope_to_function: core::ptr::null_mut(),
                root_scope: core::ptr::null_mut(),
                module: Some(module),
                dfg,
                solver_constraints: alloc::vec::Vec::new(),
                solver_constraint_limit: FInt::LuauSolverConstraintLimit.get() as usize,
                unsolved_constraints: alloc::vec::Vec::new(),
                deprecated_blocked_constraints: std::collections::HashMap::new(),
                deprecated_blocked: std::collections::HashMap::new(),
                instantiated_aliases: luaur_common::records::dense_hash_map::DenseHashMap::new(
                    empty_instantiation_signature,
                ),
                upper_bound_contributors: luaur_common::records::dense_hash_map::DenseHashMap::new(
                    core::ptr::null(),
                ),
                deprecated_type_to_constraint_set: std::collections::HashMap::new(),
                deprecated_constraint_to_mutated_types:
                    luaur_common::records::dense_hash_map::DenseHashMap::new(core::ptr::null()),
                uninhabited_type_functions:
                    luaur_common::records::dense_hash_set::DenseHashSet::new(core::ptr::null()),
                seen_constraints: luaur_common::records::dense_hash_map::DenseHashMap::new(
                    empty_subtype_constraint,
                ),
                generalized_types_: luaur_common::records::dense_hash_set::DenseHashSet::new(
                    core::ptr::null(),
                ),
                generalized_types: core::ptr::null(),
                errors: alloc::vec::Vec::new(),
                module_resolver: module_resolver as *mut ModuleResolver,
                require_cycles,
                logger,
                limits: limits.clone(),
                type_functions_to_finalize:
                    luaur_common::records::dense_hash_map::DenseHashMap::new(core::ptr::null()),
                opts: ToStringOptions {
                    exhaustive: true,
                    ..ToStringOptions::default()
                },
                cgraph,
                subtyping: subtyping as *mut Subtyping,
            };

        result.constraints = borrow_constraints(&result.constraint_set.constraints);
        result.scope_to_function = &mut result.constraint_set.scope_to_function as *mut _;
        result.root_scope = result.constraint_set.root_scope;
        result.generalized_types = &result.generalized_types_ as *const _;

        result.init_free_type_tracking();

        result
    }
}
