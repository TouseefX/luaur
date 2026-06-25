//! C++ `FragmentTypeCheckResult typecheckFragment_(...)` (FragmentAutocomplete.cpp:1119-1289).
use crate::enums::fragment_autocomplete_waypoint::FragmentAutocompleteWaypoint;
use crate::enums::solver_mode::SolverMode;
use crate::functions::borrow_constraints::borrow_constraints;
use crate::functions::clone_types_from_fragment::clone_types_from_fragment;
use crate::functions::freeze::freeze;
use crate::functions::get_module_resolver::get_module_resolver;
use crate::functions::report_waypoint::report_waypoint;
use crate::functions::trace_requires::trace_requires;
use crate::functions::unfreeze::unfreeze;
use crate::records::clone_state::CloneState;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::constraint_list::ConstraintList;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dcr_logger::DcrLogger;
use crate::records::expected_type_visitor::ExpectedTypeVisitor;
use crate::records::fragment_type_check_result::FragmentTypeCheckResult;
use crate::records::frontend::Frontend;
use crate::records::frontend_options::FrontendOptions;
use crate::records::i_fragment_autocomplete_reporter::IFragmentAutocompleteReporter;
use crate::records::module::Module;
use crate::records::module_resolver::ModuleResolver;
use crate::records::normalizer::Normalizer;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::unifier_shared_state::UnifierSharedState;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::{FFlag, FInt};

#[allow(clippy::too_many_arguments)]
pub fn typecheck_fragment_(
    frontend: &mut Frontend,
    root: *mut AstStatBlock,
    stale: &ModulePtr,
    closest_scope: &ScopePtr,
    cursor_pos: &Position,
    ast_allocator: alloc::boxed::Box<Allocator>,
    opts: &FrontendOptions,
    reporter: *mut dyn IFragmentAutocompleteReporter,
) -> FragmentTypeCheckResult {
    LUAU_TIMETRACE_SCOPE!("Luau::typecheckFragment_", "FragmentAutocomplete");
    let _ = cursor_pos;

    let stale_ptr = Arc::as_ptr(stale) as *mut Module;
    unsafe {
        freeze(&mut (*stale_ptr).internal_types);
        freeze(&mut (*stale_ptr).interface_types);
    }

    let incremental_module: ModulePtr = Arc::new(Module::default());
    let module_ptr = Arc::as_ptr(&incremental_module) as *mut Module;
    let module_name = unsafe { (*stale_ptr).name.clone() };
    unsafe {
        (*module_ptr).name = module_name.clone();
        (*module_ptr).human_readable_name =
            String::from("Incremental$") + &(*stale_ptr).human_readable_name;
        (*module_ptr).internal_types.owning_module = module_ptr;
        (*module_ptr).interface_types.owning_module = module_ptr;
        (*module_ptr).allocator = Some(Arc::from(ast_allocator));
        // C++ `Module::names` is a `std::shared_ptr<AstNameTable>`. The fragment
        // root is parsed reusing the stale module's name table
        // (`parseFragment(..., module->names.get(), ...)`,
        // FragmentAutocomplete.cpp:1317), so the incremental module's
        // constraint generation (e.g. `module->names->get(...)` in
        // `prototypeTypeDefinitions`, ConstraintGenerator.cpp:1220) resolves
        // against that same table. Share it here so the AstName lookups operate
        // on the table the fragment AST nodes were interned into.
        (*module_ptr).names = (*stale_ptr).names.clone();
        (*module_ptr).checked_in_new_solver = true;
        unfreeze(&mut (*module_ptr).internal_types);
        unfreeze(&mut (*module_ptr).interface_types);
    }

    // Setup typecheck limits
    let mut limits = TypeCheckLimits::default();
    limits.finishTime = opts
        .module_time_limit_sec
        .map(|secs| luaur_common::functions::get_clock::get_clock() + secs);
    limits.cancellationToken = opts.cancellation_token.clone();

    let ice_handler: *mut crate::records::internal_error_reporter::InternalErrorReporter =
        &mut frontend.ice_handler;
    let builtin_types = frontend.builtin_types;

    // Make the shared state for the unifier (recursion + iteration limits)
    let mut unifier_state = UnifierSharedState::unifier_shared_state(ice_handler);
    unifier_state.counters.recursion_limit = FInt::LuauTypeInferRecursionLimit.get() as i32;
    unifier_state.counters.iteration_limit = limits
        .unifierIterationLimit()
        .unwrap_or_else(|| FInt::LuauTypeInferIterationLimit.get() as i32);

    // Initialize the normalizer
    let mut normalizer = unsafe {
        Normalizer::new(
            &mut (*module_ptr).internal_types,
            builtin_types,
            &mut unifier_state,
            SolverMode::New,
            false,
        )
    };

    // User defined type functions runtime
    let mut type_function_runtime = TypeFunctionRuntime {
        ice: unsafe { (*ice_handler).clone() },
        limits: limits.clone(),
        type_arena: crate::records::typed_allocator::TypedAllocator::default(),
        type_pack_arena: crate::records::typed_allocator::TypedAllocator::default(),
        state: (core::ptr::null_mut(), None),
        initialized: DenseHashSet::new(core::ptr::null_mut()),
        allow_evaluation: false,
        root_scope: closest_scope.clone(),
        messages: Vec::new(),
        runtime_builder: core::ptr::null_mut(),
    };

    let mut subtyping = unsafe {
        Subtyping::subtyping_owned(
            builtin_types,
            &mut (*module_ptr).internal_types,
            &mut normalizer,
            &mut type_function_runtime,
            ice_handler,
        )
    };

    // Create a DataFlowGraph just for the surrounding context
    let mut dfg = unsafe {
        DataFlowGraphBuilder::build(
            root,
            &mut (*module_ptr).def_arena,
            &mut (*module_ptr).key_arena,
            ice_handler,
        )
    };
    report_waypoint(reporter, FragmentAutocompleteWaypoint::DfgBuildEnd);

    // requireTrace for the surrounding context. Erased on the way out (ScopedExit).
    let trace = trace_requires(frontend.file_resolver, root, module_name.clone(), &limits);
    frontend.require_trace.insert(module_name.clone(), trace);

    let resolver: *mut ModuleResolver =
        get_module_resolver(frontend, Some(opts.clone())) as *mut _ as *mut ModuleResolver;

    // freshChildOfNearestScope = std::make_shared<Scope>(nullptr)
    let mut fresh_scope_value = Scope::scope_type_pack_id(core::ptr::null());
    fresh_scope_value.interior_free_types = Some(Vec::new());
    fresh_scope_value.interior_free_type_packs = Some(Vec::new());
    fresh_scope_value.location = unsafe { (*root).base.base.location };
    let fresh_child_of_nearest_scope: ScopePtr = Arc::new(fresh_scope_value);
    let fresh_scope_ptr = Arc::as_ptr(&fresh_child_of_nearest_scope) as *const Scope as *mut Scope;

    let mut cgraph_storage = if FFlag::LuauConstraintGraph.get() {
        Some(ConstraintGraph {
            builtin_types: NonNull::new(builtin_types).expect("builtinTypes must not be null"),
            dependencies: DenseHashMap::new(Default::default()),
            reverse_dependencies: DenseHashMap::new(Default::default()),
            constraint_lists: Vec::<alloc::boxed::Box<ConstraintList>>::new(),
        })
    } else {
        None
    };
    let cgraph = cgraph_storage
        .as_mut()
        .map(|cgraph| cgraph as *mut ConstraintGraph)
        .unwrap_or(core::ptr::null_mut());

    let logger: *mut DcrLogger = core::ptr::null_mut();

    // Constraint Generator
    let prepare_module_scope: alloc::rc::Rc<
        dyn Fn(&crate::type_aliases::module_name_type::ModuleName, &ScopePtr),
    > = alloc::rc::Rc::new(|_, _| {});

    let mut cg = crate::records::constraint_generator::ConstraintGenerator::constraint_generator(
        incremental_module.clone(),
        NonNull::new(&mut normalizer).unwrap(),
        NonNull::new(&mut type_function_runtime).unwrap(),
        NonNull::new(resolver).expect("moduleResolver must not be null"),
        NonNull::new(builtin_types).expect("builtinTypes must not be null"),
        NonNull::new(ice_handler).expect("iceHandler must not be null"),
        frontend.globals.global_scope.clone(),
        frontend.globals.global_type_function_scope.clone(),
        prepare_module_scope,
        logger,
        NonNull::new(&mut dfg).unwrap(),
        Vec::new(),
        cgraph,
    );

    let mut clone_state = CloneState::new(unsafe { &mut *builtin_types });

    // incrementalModule->scopes.emplace_back(root->location, freshChildOfNearestScope);
    unsafe {
        (*module_ptr).scopes.push((
            (*root).base.base.location,
            fresh_child_of_nearest_scope.clone(),
        ));
    }
    cg.root_scope = fresh_scope_ptr;

    // Create module-local scope for the type function environment
    let local_type_function_scope: ScopePtr =
        Arc::new(Scope::new(cg.type_function_scope.as_ref().unwrap(), 0));
    unsafe {
        let lhs = Arc::as_ptr(&local_type_function_scope) as *const Scope as *mut Scope;
        (*lhs).location = (*root).base.base.location;
    }
    unsafe {
        (*cg.type_function_runtime).root_scope = local_type_function_scope;
    }

    report_waypoint(
        reporter,
        FragmentAutocompleteWaypoint::CloneAndSquashScopeStart,
    );
    clone_types_from_fragment(
        &mut clone_state,
        Arc::as_ptr(closest_scope),
        stale,
        unsafe { &mut (*module_ptr).internal_types },
        &mut dfg,
        builtin_types,
        root,
        fresh_scope_ptr,
    );
    report_waypoint(
        reporter,
        FragmentAutocompleteWaypoint::CloneAndSquashScopeEnd,
    );

    cg.visit_fragment_root(&fresh_child_of_nearest_scope, root);

    let cg_scopes = core::mem::take(&mut cg.scopes);
    for p in cg_scopes {
        unsafe {
            (*module_ptr).scopes.push(p);
        }
    }

    report_waypoint(
        reporter,
        FragmentAutocompleteWaypoint::ConstraintSolverStart,
    );

    // Initialize the constraint solver and run it.
    // C++ uses the fragment ConstraintSolver constructor:
    //   NotNull(cg.rootScope), borrowConstraints(cg.constraints), NotNull{&cg.scopeToFunction}, ...
    let borrowed = borrow_constraints(&cg.constraints);
    let constraints: Vec<NonNull<crate::records::constraint::Constraint>> = borrowed
        .into_iter()
        .map(|c| NonNull::new(c).expect("constraint must not be null"))
        .collect();
    let cg_root_scope = NonNull::new(cg.root_scope).expect("rootScope must not be null");
    let scope_to_function = NonNull::new(&mut cg.scope_to_function).unwrap();

    let mut cs =
        crate::records::constraint_solver::ConstraintSolver::constraint_solver_not_null_normalizer_not_null_type_function_runtime_not_null_scope_vector_not_null_constraint_not_null_dense_hash_map_scope_type_id_module_ptr_not_null_module_resolver_vector_require_cycle_dcr_logger_not_null_data_flow_graph_type_check_limits_constraint_graph_not_null_subtyping(
            &normalizer,
            &type_function_runtime,
            cg_root_scope,
            constraints,
            scope_to_function,
            incremental_module.clone(),
            resolver,
            Vec::new(),
            logger,
            &dfg,
            limits.clone(),
            cgraph,
            &subtyping,
        );

    let solver_panicked =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cs.constraint_solver_run()));
    if solver_panicked.is_err() {
        // C++ catches TimeLimitError / UserCancelError and marks the stale module.
        unsafe {
            (*stale_ptr).timeout = true;
        }
    }

    report_waypoint(reporter, FragmentAutocompleteWaypoint::ConstraintSolverEnd);

    let mut etv = unsafe {
        ExpectedTypeVisitor::new(
            &mut (*module_ptr).ast_types,
            &mut (*module_ptr).ast_expected_types,
            &mut (*module_ptr).ast_resolved_types,
            &mut (*module_ptr).ast_overload_resolved_types,
            &mut (*module_ptr).internal_types,
            builtin_types,
            fresh_scope_ptr,
        )
    };
    luaur_ast::visit::ast_stat_block_visit(unsafe { &*root }, &mut etv);

    // In frontend we would forbid internal types because this is just for autocomplete,
    // we don't actually care. We also don't even need to typecheck - just synthesize types
    // as best as we can.
    unsafe {
        freeze(&mut (*module_ptr).internal_types);
        freeze(&mut (*module_ptr).interface_types);
        (*fresh_scope_ptr).parent = Some(closest_scope.clone());
    }

    // ScopedExit: erase the requireTrace entry for the incremental module.
    frontend.require_trace.remove(&module_name);

    FragmentTypeCheckResult {
        incremental_module: Some(incremental_module),
        fresh_scope: fresh_child_of_nearest_scope,
        ancestry: Vec::new(),
    }
}
