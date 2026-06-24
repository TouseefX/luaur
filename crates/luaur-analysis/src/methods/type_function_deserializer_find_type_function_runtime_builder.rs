use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionDeserializer {
    pub fn find_type_function_type_id(&self, ty: TypeFunctionTypeId) -> Option<TypeId> {
        self.types.get(&ty).copied()
    }
}
