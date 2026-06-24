use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionSerializer {
    pub fn serialize_type_pack_id(&mut self, tp: TypePackId) -> TypeFunctionTypePackId {
        self.shallow_serialize_type_pack_id(tp);
        self.run();

        if self.has_exceeded_iteration_limit() || self.has_errors() {
            core::ptr::null()
        } else {
            self.find_type_pack_id(tp).unwrap_or(core::ptr::null())
        }
    }
}
