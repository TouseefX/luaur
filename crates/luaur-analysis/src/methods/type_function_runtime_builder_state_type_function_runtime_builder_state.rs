use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;

impl TypeFunctionRuntimeBuilderState {
    pub fn type_function_runtime_builder_state_type_function_runtime_builder_state(
        ctx: core::ptr::NonNull<TypeFunctionContext>,
    ) -> Self {
        TypeFunctionRuntimeBuilderState::new(ctx.as_ptr())
    }
}
