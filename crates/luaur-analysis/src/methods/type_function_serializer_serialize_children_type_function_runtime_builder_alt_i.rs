use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::records::type_function_union_type::TypeFunctionUnionType;
use crate::records::union_type::UnionType;

impl TypeFunctionSerializer {
    pub fn serialize_children_union_type_type_function_union_type(
        &mut self,
        u1: *const UnionType,
        u2: *mut TypeFunctionUnionType,
    ) {
        unsafe {
            let u1 = &*u1;
            let u2 = &mut *u2;

            u2.components.reserve(u1.options.len());
            for &ty in &u1.options {
                u2.components.push(self.shallow_serialize_type_id(ty));
            }
        }
    }
}
