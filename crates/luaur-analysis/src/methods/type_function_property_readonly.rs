use crate::records::type_function_property::TypeFunctionProperty;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;

impl TypeFunctionProperty {
    #[inline]
    pub fn readonly(ty: TypeFunctionTypeId) -> Self {
        TypeFunctionProperty {
            read_ty: Some(ty),
            write_ty: None,
        }
    }
}
