use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;

impl TypeFunctionCloner {
    pub fn clone_type_function_type_pack_id(
        &mut self,
        tp: TypeFunctionTypePackId,
    ) -> TypeFunctionTypePackId {
        self.shallow_clone_type_function_type_pack_id(tp);
        self.run();

        if self.has_exceeded_iteration_limit() {
            return core::ptr::null();
        }

        self.find_type_function_type_pack_id(tp)
            .unwrap_or(core::ptr::null())
    }
}
