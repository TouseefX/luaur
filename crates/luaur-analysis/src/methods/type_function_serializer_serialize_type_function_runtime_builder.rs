use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionSerializer {
    pub fn serialize_type_id(&mut self, ty: TypeId) -> TypeFunctionTypeId {
        self.shallow_serialize_type_id(ty);
        self.run();

        if self.has_exceeded_iteration_limit() || self.has_errors() {
            core::ptr::null()
        } else {
            self.find_type_id(ty).unwrap_or(core::ptr::null())
        }
    }
}
