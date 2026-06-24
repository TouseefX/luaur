#[derive(Debug, Clone)]
pub struct TypeFunctionUnionType {
    pub(crate) components:
        alloc::vec::Vec<crate::type_aliases::type_function_type_id::TypeFunctionTypeId>,
}
