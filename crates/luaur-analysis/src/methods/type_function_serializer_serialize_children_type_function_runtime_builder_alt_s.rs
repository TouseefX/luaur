use crate::records::generic_type_pack::GenericTypePack;
use crate::records::type_function_generic_type_pack::TypeFunctionGenericTypePack;
use crate::records::type_function_serializer::TypeFunctionSerializer;

impl TypeFunctionSerializer {
    pub fn serialize_children_generic_type_pack_type_function_generic_type_pack(
        &mut self,
        _v1: *const GenericTypePack,
        _v2: *mut TypeFunctionGenericTypePack,
    ) {
    }
}
