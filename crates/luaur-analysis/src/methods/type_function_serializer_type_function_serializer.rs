use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::type_aliases::seen_type_packs_type_function_runtime_builder::SeenTypePacks;
use crate::type_aliases::seen_types_type_function_runtime_builder::SeenTypes;

impl TypeFunctionSerializer {
    pub fn type_function_serializer(&mut self, state: *mut TypeFunctionRuntimeBuilderState) {
        self.state = state;
        self.type_function_runtime = unsafe {
            (*state)
                .ctx
                .as_ref()
                .unwrap()
                .type_function_runtime
                .as_ptr()
        };
        self.queue = alloc::vec::Vec::new();
        self.types = SeenTypes::new(core::ptr::null());
        self.packs = SeenTypePacks::new(core::ptr::null());
        self.steps = 0;
    }
}
