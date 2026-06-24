//! C++ `TypeFunctionContext::TypeFunctionContext(NotNull<TypeArena>,
//! NotNull<BuiltinTypes>, NotNull<Scope>, NotNull<Normalizer>,
//! NotNull<TypeFunctionRuntime>, NotNull<InternalErrorReporter>,
//! NotNull<TypeCheckLimits>, NotNull<Subtyping>)` (TypeFunction.h:60-81). Plain
//! field-init ctor; `solver`/`constraint` are null because this overload is
//! used when reducing outside of the constraint solver.
use core::ptr::NonNull;

use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_runtime::TypeFunctionRuntime;

impl TypeFunctionContext {
    /// C++ ctor used when reducing outside of the constraint solver.
    #[allow(clippy::too_many_arguments)]
    pub fn from_components(
        arena: NonNull<TypeArena>,
        builtins: NonNull<BuiltinTypes>,
        scope: NonNull<Scope>,
        normalizer: NonNull<Normalizer>,
        type_function_runtime: NonNull<TypeFunctionRuntime>,
        ice: NonNull<InternalErrorReporter>,
        limits: NonNull<TypeCheckLimits>,
        subtyping: NonNull<Subtyping>,
    ) -> Self {
        TypeFunctionContext {
            arena,
            builtins,
            scope,
            normalizer,
            type_function_runtime,
            ice,
            limits,
            subtyping,
            solver: core::ptr::null_mut(),
            constraint: core::ptr::null(),
            user_func_name: None,
            fresh_instances: alloc::vec::Vec::new(),
        }
    }
}
