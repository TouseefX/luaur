use crate::enums::solver_mode::SolverMode;
use crate::functions::check_non_strict::check_non_strict;
use crate::functions::check_type_checker_2::check as check_type_checker_2;
use crate::functions::freeze::freeze;
use crate::functions::synthesize_export_return::synthesize_export_return;
use crate::functions::unfreeze::unfreeze;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::constraint_list::ConstraintList;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dcr_logger::DcrLogger;
use crate::records::file_resolver::FileResolver;
use crate::records::frontend_options::FrontendOptions;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module::Module;
use crate::records::module_resolver::ModuleResolver;
use crate::records::normalizer::Normalizer;
use crate::records::require_cycle::RequireCycle;
use crate::records::source_module::SourceModule;
use crate::records::stats::Stats;
use crate::records::subtyping::Subtyping;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::unifier_shared_state::UnifierSharedState;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::enums::mode::Mode;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::{FFlag, FInt};
use std::rc::Rc;

pub fn check(
    source_module: &SourceModule,
    mode: Mode,
    require_cycles: &[RequireCycle],
    builtin_types: *mut BuiltinTypes,
    ice_handler: *mut InternalErrorReporter,
    module_resolver: *mut ModuleResolver,
    _file_resolver: *mut FileResolver,
    parent_scope: &ScopePtr,
    type_function_scope: &ScopePtr,
    prepare_module_scope: Rc<dyn Fn(&ModuleName, &ScopePtr)>,
    options: FrontendOptions,
    limits: TypeCheckLimits,
    _record_json_log: bool,
    stats: &mut Stats,
    _write_json_log: Rc<dyn Fn(&ModuleName, String)>,
) -> ModulePtr {
    let module: ModulePtr = Arc::new(Module::default());
    let module_ptr = Arc::as_ptr(&module) as *mut Module;

    unsafe {
        (*module_ptr).checked_in_new_solver = true;
        (*module_ptr).name = source_module.name.clone();
        (*module_ptr).human_readable_name = source_module.human_readable_name.clone();
        (*module_ptr).mode = mode;
        (*module_ptr).internal_types.owning_module = module_ptr;
        (*module_ptr).interface_types.owning_module = module_ptr;
        (*module_ptr).internal_types.collect_singleton_stats =
            options.collect_type_allocation_stats;
        (*module_ptr).allocator = Some(source_module.allocator.clone());
        (*module_ptr).names = Some(source_module.names.clone());
        (*module_ptr).root = source_module.root;
        (*ice_handler).module_name = source_module.name.clone();
    }

    let mut dfg = unsafe {
        DataFlowGraphBuilder::build(
            source_module.root,
            &mut (*module_ptr).def_arena,
            &mut (*module_ptr).key_arena,
            ice_handler,
        )
    };

    let mut unifier_state = UnifierSharedState::unifier_shared_state(ice_handler);
    unifier_state.counters.recursion_limit = FInt::LuauTypeInferRecursionLimit.get() as i32;
    unifier_state.counters.iteration_limit = limits
        .unifierIterationLimit()
        .unwrap_or_else(|| FInt::LuauTypeInferIterationLimit.get() as i32);

    let mut normalizer = unsafe {
        Normalizer::new(
            &mut (*module_ptr).internal_types,
            builtin_types,
            &mut unifier_state,
            SolverMode::New,
            false,
        )
    };

    let mut type_function_runtime = TypeFunctionRuntime {
        ice: unsafe { (*ice_handler).clone() },
        limits: limits.clone(),
        type_arena: crate::records::typed_allocator::TypedAllocator::default(),
        type_pack_arena: crate::records::typed_allocator::TypedAllocator::default(),
        state: (core::ptr::null_mut(), None),
        initialized: DenseHashSet::new(core::ptr::null_mut()),
        allow_evaluation: true,
        root_scope: parent_scope.clone(),
        messages: Vec::new(),
        runtime_builder: core::ptr::null_mut(),
    };

    let mut cgraph_storage = if FFlag::LuauConstraintGraph.get() {
        Some(ConstraintGraph {
            builtin_types: NonNull::new(builtin_types).expect("builtinTypes must not be null"),
            dependencies: DenseHashMap::new(Default::default()),
            reverse_dependencies: DenseHashMap::new(Default::default()),
            constraint_lists: Vec::<Box<ConstraintList>>::new(),
        })
    } else {
        None
    };
    let cgraph = cgraph_storage
        .as_mut()
        .map(|cgraph| cgraph as *mut ConstraintGraph)
        .unwrap_or(core::ptr::null_mut());

    let mut subtyping = unsafe {
        Subtyping::subtyping_owned(
            builtin_types,
            &mut (*module_ptr).internal_types,
            &mut normalizer,
            &mut type_function_runtime,
            ice_handler,
        )
    };

    let logger: *mut DcrLogger = core::ptr::null_mut();
    let mut cg = crate::records::constraint_generator::ConstraintGenerator::constraint_generator(
        module.clone(),
        NonNull::new(&mut normalizer).unwrap(),
        NonNull::new(&mut type_function_runtime).unwrap(),
        NonNull::new(module_resolver).expect("moduleResolver must not be null"),
        NonNull::new(builtin_types).expect("builtinTypes must not be null"),
        NonNull::new(ice_handler).expect("iceHandler must not be null"),
        parent_scope.clone(),
        type_function_scope.clone(),
        prepare_module_scope,
        logger,
        NonNull::new(&mut dfg).unwrap(),
        require_cycles.to_vec(),
        cgraph,
    );

    let constraint_set = cg.run(source_module.root);
    unsafe {
        (*module_ptr).errors = constraint_set.errors.clone();
        (*module_ptr).constraint_generation_did_not_complete = cg.recursion_limit_met;
    }

    let mut cs =
        crate::records::constraint_solver::ConstraintSolver::constraint_solver_not_null_normalizer_not_null_type_function_runtime_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
            &normalizer,
            &type_function_runtime,
            module.clone(),
            module_resolver,
            require_cycles.to_vec(),
            logger,
            &dfg,
            limits.clone(),
            constraint_set,
            cgraph,
            &subtyping,
        );

    if let Some(seed) = options.randomize_constraint_resolution_seed {
        cs.randomize(seed);
    }

    cs.constraint_solver_run();
    stats.dynamic_constraints_created += cs.solver_constraints.len();

    unsafe {
        (*module_ptr).errors.extend(cs.errors.iter().cloned());
        (*module_ptr).scopes = core::mem::take(&mut cg.scopes);
        (*module_ptr).r#type = source_module.r#type;
        (*module_ptr).upper_bound_contributors = core::mem::replace(
            &mut cs.upper_bound_contributors,
            DenseHashMap::new(core::ptr::null()),
        );
    }

    if !unsafe { (*module_ptr).timeout || (*module_ptr).cancelled } {
        match mode {
            Mode::Nonstrict => {
                check_non_strict(
                    builtin_types,
                    &mut type_function_runtime,
                    ice_handler,
                    &mut unifier_state,
                    &dfg,
                    &mut limits.clone(),
                    source_module,
                    module_ptr,
                );
            }
            Mode::Definition | Mode::Strict => {
                check_type_checker_2(
                    builtin_types,
                    &mut type_function_runtime,
                    &mut unifier_state,
                    &mut limits.clone(),
                    logger,
                    source_module,
                    module_ptr,
                );
            }
            Mode::NoCheck => {}
        }

        if FFlag::LuauExportValueSyntax.get()
            && FFlag::LuauExportValueTypecheck.get()
            && !unsafe { (*module_ptr).timeout || (*module_ptr).cancelled }
        {
            synthesize_export_return(builtin_types, module_ptr);
        }
    }

    unsafe {
        if (*module_ptr).errors.len() == 1
            && !FFlag::DebugLuauAlwaysShowConstraintSolvingIncomplete.get()
            && matches!(
                &(&(*module_ptr).errors)[0].data,
                crate::type_aliases::type_error_data::TypeErrorData::ConstraintSolvingIncompleteError(_)
            )
        {
            (*module_ptr).errors.clear();
        }

        unfreeze(&mut (*module_ptr).interface_types);
        (*module_ptr).clone_public_interface(builtin_types, &mut *ice_handler, SolverMode::New);
        freeze(&mut (*module_ptr).internal_types);
        freeze(&mut (*module_ptr).interface_types);
    }

    module
}
