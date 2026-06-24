use crate::enums::solver_mode::SolverMode;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::hash_bool_name_pair::HashBoolNamePair;
use crate::records::instantiation::Instantiation;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module_resolver::ModuleResolver;
use crate::records::normalizer::Normalizer;
use crate::records::txn_log::TxnLog;
use crate::records::type_checker::TypeChecker;
use crate::records::type_level::TypeLevel;
use crate::records::unifier_shared_state::UnifierSharedState;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeChecker {
    pub fn new(
        global_scope: &ScopePtr,
        resolver: *mut ModuleResolver,
        builtin_types: *mut BuiltinTypes,
        ice_handler: *mut InternalErrorReporter,
    ) -> Self {
        let unifier_state = UnifierSharedState::unifier_shared_state(ice_handler);

        let mut result = TypeChecker {
            global_scope: global_scope as *const ScopePtr,
            resolver,
            current_module: None,
            builtin_types,
            ice_handler,
            unifier_state,
            normalizer: Normalizer::new(
                core::ptr::null_mut(),
                builtin_types,
                core::ptr::null_mut(),
                SolverMode::Old,
                false,
            ),
            reusable_instantiation: Instantiation::instantiation_new(
                TxnLog::empty(),
                core::ptr::null_mut(),
                builtin_types,
                TypeLevel::default(),
                core::ptr::null_mut(),
            ),
            require_cycles: Vec::new(),
            finish_time: None,
            instantiation_child_limit: None,
            unifier_iteration_limit: None,
            cancellation_token: None,
            prepare_module_scope: None,
            nil_type: unsafe { (*builtin_types).nilType },
            number_type: unsafe { (*builtin_types).numberType },
            integer_type: unsafe { (*builtin_types).integerType },
            string_type: unsafe { (*builtin_types).stringType },
            boolean_type: unsafe { (*builtin_types).booleanType },
            thread_type: unsafe { (*builtin_types).threadType },
            buffer_type: unsafe { (*builtin_types).bufferType },
            any_type: unsafe { (*builtin_types).anyType },
            unknown_type: unsafe { (*builtin_types).unknownType },
            never_type: unsafe { (*builtin_types).neverType },
            any_type_pack: unsafe { (*builtin_types).anyTypePack },
            never_type_pack: unsafe { (*builtin_types).neverTypePack },
            uninhabitable_type_pack: unsafe { (*builtin_types).uninhabitableTypePack },
            check_recursion_count: 0,
            recursion_count: 0,
            duplicate_type_aliases: DenseHashSet::new((false, alloc::string::String::new())),
            incorrect_extern_type_definitions: DenseHashSet::new(core::ptr::null()),
            deferred_quantification: Vec::new(),
        };

        result.normalizer.shared_state = &mut result.unifier_state;
        result
    }

    pub fn type_checker_scope_ptr_module_resolver_not_null_builtin_types_internal_error_reporter(
        &mut self,
        global_scope: &ScopePtr,
        resolver: *mut ModuleResolver,
        builtin_types: *mut BuiltinTypes,
        ice_handler: *mut InternalErrorReporter,
    ) {
        *self = Self::new(global_scope, resolver, builtin_types, ice_handler);
    }
}
