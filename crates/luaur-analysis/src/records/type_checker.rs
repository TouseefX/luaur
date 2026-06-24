//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/TypeInfer.h:62:type_checker`
//! Source: `Analysis/include/Luau/TypeInfer.h` (hand-ported; fields only)

use crate::records::builtin_types::BuiltinTypes;
use crate::records::frontend_cancellation_token::FrontendCancellationToken;
use crate::records::hash_bool_name_pair::HashBoolNamePair;
use crate::records::instantiation::Instantiation;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module_resolver::ModuleResolver;
use crate::records::normalizer::Normalizer;
use crate::records::require_cycle::RequireCycle;
use crate::records::unifier_shared_state::UnifierSharedState;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub struct TypeChecker {
    pub global_scope: *const ScopePtr, // const ScopePtr&
    pub resolver: *mut ModuleResolver,
    pub current_module: Option<ModulePtr>,
    pub builtin_types: *mut BuiltinTypes, // NotNull<BuiltinTypes>
    pub ice_handler: *mut InternalErrorReporter,
    pub unifier_state: UnifierSharedState,
    pub normalizer: Normalizer,
    pub reusable_instantiation: Instantiation,
    pub require_cycles: Vec<RequireCycle>,
    pub finish_time: Option<f64>,
    pub instantiation_child_limit: Option<i32>,
    pub unifier_iteration_limit: Option<i32>,
    pub cancellation_token: Option<Arc<FrontendCancellationToken>>,
    pub prepare_module_scope: Option<Rc<dyn Fn(&ModuleName, &ScopePtr)>>,

    pub nil_type: TypeId,
    pub number_type: TypeId,
    pub integer_type: TypeId,
    pub string_type: TypeId,
    pub boolean_type: TypeId,
    pub thread_type: TypeId,
    pub buffer_type: TypeId,
    pub any_type: TypeId,
    pub unknown_type: TypeId,
    pub never_type: TypeId,

    pub any_type_pack: TypePackId,
    pub never_type_pack: TypePackId,
    pub uninhabitable_type_pack: TypePackId,

    pub check_recursion_count: i32,
    pub recursion_count: i32,

    pub duplicate_type_aliases: DenseHashSet<(bool, Name), HashBoolNamePair>,
    pub incorrect_extern_type_definitions: DenseHashSet<*const AstStatDeclareExternType>,
    pub deferred_quantification: Vec<(TypeId, ScopePtr)>,
}

impl core::fmt::Debug for TypeChecker {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TypeChecker")
            .field("global_scope", &self.global_scope)
            .field("resolver", &self.resolver)
            .field("current_module", &self.current_module)
            .field("builtin_types", &self.builtin_types)
            .field("ice_handler", &self.ice_handler)
            .field("require_cycles", &self.require_cycles)
            .field("finish_time", &self.finish_time)
            .field("instantiation_child_limit", &self.instantiation_child_limit)
            .field("unifier_iteration_limit", &self.unifier_iteration_limit)
            .field("cancellation_token", &self.cancellation_token)
            .field(
                "prepare_module_scope",
                &self.prepare_module_scope.as_ref().map(|_| "..."),
            )
            .field("check_recursion_count", &self.check_recursion_count)
            .field("recursion_count", &self.recursion_count)
            .finish()
    }
}
