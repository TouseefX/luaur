use crate::enums::solver_mode::SolverMode;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module::Module;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::normalizer::Normalizer;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::unifier_shared_state::UnifierSharedState;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl NonStrictTypeChecker {
    pub fn non_strict_type_checker(
        arena: *mut TypeArena,
        builtin_types: *mut BuiltinTypes,
        type_function_runtime: *mut TypeFunctionRuntime,
        ice: *const InternalErrorReporter,
        unifier_state: *mut UnifierSharedState,
        dfg: *const DataFlowGraph,
        limits: *const TypeCheckLimits,
        module: *mut Module,
    ) -> Self {
        let normalizer =
            Normalizer::new(arena, builtin_types, unifier_state, SolverMode::New, true);
        let subtyping = Subtyping::subtyping_owned(
            builtin_types,
            arena,
            core::ptr::null_mut(),
            type_function_runtime,
            ice as *mut InternalErrorReporter,
        );

        NonStrictTypeChecker {
            builtin_types,
            type_function_runtime,
            ice: ice as *mut InternalErrorReporter,
            arena,
            module,
            normalizer,
            subtyping,
            dfg,
            no_type_function_errors: DenseHashSet::new(core::ptr::null()),
            stack: Vec::new(),
            cached_negations: DenseHashMap::new(core::ptr::null()),
            limits: limits as *mut TypeCheckLimits,
            non_strict_recursion_count: 0,
        }
    }

    /// Wires `subtyping.normalizer` to the embedded `normalizer` after this
    /// checker has moved into its final stack slot.
    ///
    /// # Safety
    /// The checker must not be moved after this call.
    pub unsafe fn wire_self_pointers(&mut self) {
        self.subtyping.normalizer = &mut self.normalizer as *mut Normalizer;
    }
}
