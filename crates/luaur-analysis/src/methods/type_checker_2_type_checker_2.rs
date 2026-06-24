use crate::enums::solver_mode::SolverMode;
use crate::enums::type_context::TypeContext;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::dcr_logger::DcrLogger;
use crate::records::module::Module;
use crate::records::normalizer::Normalizer;
use crate::records::source_module::SourceModule;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::unifier_shared_state::UnifierSharedState;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeChecker2 {
    /// C++ `TypeChecker2::TypeChecker2(NotNull<BuiltinTypes>,
    /// NotNull<TypeFunctionRuntime>, NotNull<UnifierSharedState>,
    /// NotNull<TypeCheckLimits>, DcrLogger*, const SourceModule*, Module*)`
    /// (`Analysis/src/TypeChecker2.cpp:307`).
    ///
    /// Owned constructor. The C++ member-init list wires two self-referential
    /// pointers — `_subtyping`'s `NotNull<Normalizer>` points at the embedded
    /// `normalizer`, and `subtyping` points at the embedded `_subtyping`. Those
    /// cannot be set here because the returned value is moved into its final
    /// slot, so they are left null and wired by [`TypeChecker2::wire_self_pointers`]
    /// once the `TypeChecker2` lives at a stable address.
    pub fn new(
        builtin_types: *mut BuiltinTypes,
        type_function_runtime: *mut TypeFunctionRuntime,
        unifier_state: *mut UnifierSharedState,
        limits: *mut TypeCheckLimits,
        logger: *mut DcrLogger,
        source_module: *const SourceModule,
        module: *mut Module,
    ) -> Self {
        // ice(unifierState->iceHandler)
        let ice = unsafe { (*unifier_state).ice_handler };

        // &module->internalTypes
        let arena: *mut TypeArena = unsafe { &mut (*module).internal_types };

        // normalizer{&module->internalTypes, builtinTypes, unifierState, SolverMode::New, /* cacheInhabitance */ true}
        let normalizer =
            Normalizer::new(arena, builtin_types, unifier_state, SolverMode::New, true);

        // _subtyping{builtinTypes, NotNull{&module->internalTypes}, NotNull{&normalizer},
        //            typeFunctionRuntime, NotNull{unifierState->iceHandler}}
        // The NotNull<Normalizer> is wired in `wire_self_pointers` (it must point
        // at the moved-in `normalizer` field).
        let _subtyping = Subtyping::subtyping_owned(
            builtin_types,
            arena,
            core::ptr::null_mut(),
            type_function_runtime,
            ice,
        );

        TypeChecker2 {
            builtin_types,
            type_function_runtime,
            logger,
            limits,
            ice,
            source_module,
            module,
            type_context: TypeContext::default(),
            stack: Vec::new(),
            function_decl_stack: Vec::new(),
            seen_type_function_instances: DenseHashSet::new(core::ptr::null()),
            normalizer,
            _subtyping,
            // subtyping(&_subtyping) — wired in wire_self_pointers.
            subtyping: core::ptr::null_mut(),
            warned_globals: DenseHashSet::new(String::new()),
        }
    }

    /// Wires the two self-referential pointers the C++ member-init list sets:
    /// `_subtyping.normalizer = &normalizer` and `subtyping = &_subtyping`.
    /// Must be called after the `TypeChecker2` is at its final address (i.e.
    /// after the `new(..)` value has been moved into its storage slot) and
    /// before any use of `subtyping`.
    ///
    /// # Safety
    /// The `TypeChecker2` must not be moved after this call, or the wired
    /// pointers dangle.
    pub unsafe fn wire_self_pointers(&mut self) {
        let normalizer_ptr: *mut Normalizer = &mut self.normalizer;
        self._subtyping.normalizer = normalizer_ptr;
        self.subtyping = &mut self._subtyping as *mut Subtyping;
    }

    pub fn type_checker_2_type_checker_2_not_null_builtin_types_not_null_type_function_runtime_not_null_unifier_shared_state_not_null_type_check_limits_dcr_logger_not_null_source_module_not_null_module(
        &mut self,
        builtin_types: *mut BuiltinTypes,
        type_function_runtime: *mut TypeFunctionRuntime,
        unifier_state: *mut UnifierSharedState,
        limits: *mut TypeCheckLimits,
        logger: *mut DcrLogger,
        source_module: *const SourceModule,
        module: *mut Module,
    ) {
        self.builtin_types = builtin_types;
        self.type_function_runtime = type_function_runtime;
        self.logger = logger;
        self.limits = limits;
        // ice(unifierState->iceHandler)
        self.ice = unsafe { (*unifier_state).ice_handler };
        self.source_module = source_module;
        self.module = module;

        let arena: *mut crate::records::type_arena::TypeArena =
            unsafe { &mut (*module).internal_types };

        // normalizer{&module->internalTypes, builtinTypes, unifierState, SolverMode::New, /* cacheInhabitance */ true}
        self.normalizer
            .normalizer_type_arena_not_null_builtin_types_not_null_unifier_shared_state_solver_mode_bool(
                arena,
                builtin_types,
                unifier_state,
                SolverMode::New,
                true,
            );

        // _subtyping{builtinTypes, NotNull{&module->internalTypes}, NotNull{&normalizer}, typeFunctionRuntime, NotNull{unifierState->iceHandler}}
        let normalizer_ptr: *mut crate::records::normalizer::Normalizer = &mut self.normalizer;
        let ice_handler = unsafe { (*unifier_state).ice_handler };
        self._subtyping
            .subtyping_not_null_builtin_types_not_null_type_arena_not_null_normalizer_not_null_type_function_runtime_not_null_internal_error_reporter(
                builtin_types,
                arena,
                normalizer_ptr,
                type_function_runtime,
                ice_handler,
            );

        // subtyping(&_subtyping)
        self.subtyping = &mut self._subtyping;
    }
}
