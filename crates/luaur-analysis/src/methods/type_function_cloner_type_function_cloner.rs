use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::records::type_function_runtime::TypeFunctionRuntime;

impl TypeFunctionCloner {
    pub fn new(runtime: *mut TypeFunctionRuntime) -> Self {
        Self {
            type_function_runtime: runtime,
            queue: alloc::vec::Vec::new(),
            types: crate::type_aliases::seen_types_type_function_runtime::SeenTypes::new(
                core::ptr::null(),
            ),
            packs: crate::type_aliases::seen_type_packs_type_function_runtime::SeenTypePacks::new(
                core::ptr::null(),
            ),
            steps: 0,
        }
    }
}
