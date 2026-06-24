use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::dcr_logger::DcrLogger;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module_resolver::ModuleResolver;
use crate::records::normalizer::Normalizer;
use crate::records::require_cycle::RequireCycle;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn constraint_generator(
        module: ModulePtr,
        normalizer: NonNull<Normalizer>,
        type_function_runtime: NonNull<TypeFunctionRuntime>,
        module_resolver: NonNull<ModuleResolver>,
        builtin_types: NonNull<BuiltinTypes>,
        ice: NonNull<InternalErrorReporter>,
        global_scope: ScopePtr,
        type_function_scope: ScopePtr,
        prepare_module_scope: Rc<dyn Fn(&ModuleName, &ScopePtr)>,
        logger: *mut DcrLogger,
        dfg: NonNull<DataFlowGraph>,
        require_cycles: Vec<RequireCycle>,
        cgraph: *mut ConstraintGraph,
    ) -> Self {
        let normalizer_ref = unsafe { normalizer.as_ref() };
        let arena = unsafe { (*normalizer_ref).arena };

        let result = ConstraintGenerator {
            scopes: Vec::new(),
            module: Some(module),
            builtin_types: builtin_types.as_ptr(),
            arena,
            root_scope: core::ptr::null_mut(),
            type_context: crate::enums::type_context::TypeContext::default(),
            inferred_bindings: luaur_common::records::dense_hash_map::DenseHashMap::new(
                crate::records::symbol::Symbol::default(),
            ),
            constraints: Vec::new(),
            free_types: crate::records::type_ids::TypeIds::type_ids(),
            scope_to_function: luaur_common::records::dense_hash_map::DenseHashMap::new(
                core::ptr::null_mut(),
            ),
            ast_type_alias_defining_scopes: luaur_common::records::dense_hash_map::DenseHashMap::new(
                core::ptr::null(),
            ),
            dfg: dfg.as_ptr(),
            refinement_arena: crate::records::refinement_arena_refinement::RefinementArena {
                allocator: crate::records::typed_allocator::TypedAllocator::default(),
            },
            recursion_count: 0,
            errors: Vec::new(),
            normalizer: normalizer.as_ptr(),
            type_function_runtime: type_function_runtime.as_ptr(),
            ast_type_function_environment_scopes:
                luaur_common::records::dense_hash_map::DenseHashMap::new(core::ptr::null()),
            module_resolver: module_resolver.as_ptr(),
            ice: ice.as_ptr(),
            global_scope: Some(global_scope),
            type_function_scope: Some(type_function_scope),
            prepare_module_scope,
            require_cycles,
            local_types: luaur_common::records::dense_hash_map::DenseHashMap::new(core::ptr::null()),
            inferred_expr_cache: luaur_common::records::dense_hash_map::DenseHashMap::new(
                core::ptr::null_mut(),
            ),
            class_decl_records: luaur_common::records::dense_hash_map::DenseHashMap::new(
                core::ptr::null_mut(),
            ),
            logger,
            recursion_limit_met: false,
            cgraph,
            interior_free_types: Vec::new(),
            unions_to_simplify: Vec::new(),
            uninitialized_globals: crate::records::set::Set::new(
                luaur_ast::records::ast_name::AstName::default(),
            ),
            polarity: crate::enums::polarity::Polarity::default(),
            prop_index_pairs_seen: luaur_common::records::dense_hash_map::DenseHashMap::new((
                core::ptr::null(),
                alloc::string::String::new(),
            )),
            large_table_depth: 0,
        };

        LUAU_ASSERT!(result.module.is_some());

        result
    }
}
