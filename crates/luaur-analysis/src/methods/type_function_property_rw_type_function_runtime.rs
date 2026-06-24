use crate::records::type_function_property::TypeFunctionProperty;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;

impl TypeFunctionProperty {
    pub fn rw_type_function_type_id(&self, ty: TypeFunctionTypeId) -> Self {
        self.rw_type_function_type_id_type_function_type_id(ty, ty)
    }
}
