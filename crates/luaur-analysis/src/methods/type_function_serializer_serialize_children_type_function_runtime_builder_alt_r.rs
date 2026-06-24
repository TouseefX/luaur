use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;
use crate::records::variadic_type_pack::VariadicTypePack;

impl TypeFunctionSerializer {
    pub fn serialize_children_variadic_type_pack_type_function_variadic_type_pack(
        &mut self,
        v1: *const VariadicTypePack,
        v2: *mut TypeFunctionVariadicTypePack,
    ) {
        unsafe {
            let v1 = &*v1;
            let v2 = &mut *v2;
            v2.type_id = self.shallow_serialize_type_id(v1.ty);
        }
    }
}
