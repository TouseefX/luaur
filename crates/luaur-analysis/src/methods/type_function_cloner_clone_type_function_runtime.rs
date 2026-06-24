use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;

impl TypeFunctionCloner {
    pub fn clone_type_function_type_id(&mut self, ty: TypeFunctionTypeId) -> TypeFunctionTypeId {
        self.shallow_clone_type_function_type_id(ty);
        self.run();

        if self.has_exceeded_iteration_limit() {
            return core::ptr::null();
        }

        self.find_type_function_type_id(ty)
            .unwrap_or(core::ptr::null())
    }
}
