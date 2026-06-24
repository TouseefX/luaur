use crate::records::type_function_property::TypeFunctionProperty;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;

impl TypeFunctionProperty {
    pub fn rw_type_function_type_id_type_function_type_id(
        &self,
        read: TypeFunctionTypeId,
        write: TypeFunctionTypeId,
    ) -> Self {
        TypeFunctionProperty {
            read_ty: Some(read),
            write_ty: Some(write),
        }
    }
}
