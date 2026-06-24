use crate::records::intersection_type::IntersectionType;
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;

impl TypeFunctionDeserializer {
    pub fn deserialize_children_type_function_intersection_type_intersection_type(
        &mut self,
        i2: *mut TypeFunctionIntersectionType,
        i1: *mut IntersectionType,
    ) {
        unsafe {
            for ty in &(*i2).components {
                let ty_id = self.shallow_deserialize_type_function_type_id(*ty);
                (*i1).parts.push(ty_id);
            }
        }
    }
}
