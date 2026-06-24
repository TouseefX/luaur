use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::records::type_function_type::TypeFunctionType;
use crate::records::type_function_type_pack_var::TypeFunctionTypePackVar;
use crate::records::typed_allocator::TypedAllocator;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::state_ref::StateRef;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::ast_stat_type_function::AstStatTypeFunction;
use luaur_common::records::dense_hash_set::DenseHashSet;

// Non-copyable in C++ (owns TypedAllocator arenas) — Debug only.
#[derive(Debug)]
pub struct TypeFunctionRuntime {
    pub(crate) ice: InternalErrorReporter,
    pub(crate) limits: TypeCheckLimits,
    pub(crate) type_arena: TypedAllocator<TypeFunctionType>,
    pub(crate) type_pack_arena: TypedAllocator<TypeFunctionTypePackVar>,
    pub(crate) state: StateRef,
    pub(crate) initialized: DenseHashSet<*mut AstStatTypeFunction>,
    pub(crate) allow_evaluation: bool,
    pub(crate) root_scope: ScopePtr,
    pub(crate) messages: Vec<String>,
    pub(crate) runtime_builder: *mut TypeFunctionRuntimeBuilderState,
}
