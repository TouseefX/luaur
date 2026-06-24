use crate::records::intersection_type::IntersectionType;
use crate::records::type_function_intersection_type::TypeFunctionIntersectionType;
use crate::records::type_function_serializer::TypeFunctionSerializer;

impl TypeFunctionSerializer {
    pub fn serialize_children_intersection_type_type_function_intersection_type(
        &mut self,
        i1: *const IntersectionType,
        i2: *mut TypeFunctionIntersectionType,
    ) {
        unsafe {
            let i1 = &*i1;
            let i2 = &mut *i2;

            i2.components.reserve(i1.parts.len());
            for &ty in &i1.parts {
                i2.components.push(self.shallow_serialize_type_id(ty));
            }
        }
    }
}
