use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_union_type::TypeFunctionUnionType;
use crate::records::union_type::UnionType;

impl TypeFunctionDeserializer {
    pub fn deserialize_children_type_function_union_type_union_type(
        &mut self,
        u2: *mut TypeFunctionUnionType,
        u1: *mut UnionType,
    ) {
        unsafe {
            for ty in &(*u2).components {
                let ty_id = self.shallow_deserialize_type_function_type_id(*ty);
                (*u1).options.push(ty_id);
            }
        }
    }
}
