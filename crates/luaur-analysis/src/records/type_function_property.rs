#[derive(Debug, Clone)]
pub struct TypeFunctionProperty {
    pub(crate) read_ty: Option<crate::type_aliases::type_function_type_id::TypeFunctionTypeId>,
    pub(crate) write_ty: Option<crate::type_aliases::type_function_type_id::TypeFunctionTypeId>,
}
