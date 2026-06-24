use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;

impl TypeFunctionDeserializer {
    pub fn type_function_deserializer(&mut self, state: *mut TypeFunctionRuntimeBuilderState) {
        self.state = state;
        self.type_function_runtime = unsafe {
            (*state)
                .ctx
                .as_ref()
                .unwrap()
                .type_function_runtime
                .as_ptr()
        };
        self.queue = Vec::new();
        self.types =
            crate::type_aliases::seen_types_type_function_runtime_builder_alt_d::SeenTypes::new(
                core::ptr::null_mut(),
            );
        self.packs = crate::type_aliases::seen_type_packs_type_function_runtime_builder_alt_d::SeenTypePacks::new(core::ptr::null_mut());
        self.generic_types = Vec::new();
        self.generic_packs = Vec::new();
        self.function_scopes = Vec::new();
        self.steps = 0;
    }
}
