use crate::functions::follow_type::follow_type_id;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionSerializer {
    pub fn find_type_id(&self, ty: TypeId) -> Option<TypeFunctionTypeId> {
        let ty = unsafe { follow_type_id(ty) };
        self.types.get(&ty).copied()
    }
}
