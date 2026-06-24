use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::scope::Scope;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::typed_allocator::TypedAllocator;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::sync::Arc;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl TypeFunctionRuntime {
    pub fn new(
        ice: &InternalErrorReporter,
        limits: &TypeCheckLimits,
        root_scope: ScopePtr,
    ) -> Self {
        Self {
            ice: ice.clone(),
            limits: limits.clone(),
            type_arena: TypedAllocator::default(),
            type_pack_arena: TypedAllocator::default(),
            state: (core::ptr::null_mut(), None),
            initialized: DenseHashSet::new(core::ptr::null_mut()),
            allow_evaluation: true,
            root_scope,
            messages: Vec::new(),
            runtime_builder: core::ptr::null_mut(),
        }
    }

    pub fn new_with_empty_root(ice: &InternalErrorReporter, limits: &TypeCheckLimits) -> Self {
        Self::new(
            ice,
            limits,
            Arc::new(Scope::scope_type_pack_id(core::ptr::null())),
        )
    }
}
