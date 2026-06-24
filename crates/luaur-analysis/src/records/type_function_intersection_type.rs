#[derive(Debug, Clone)]
pub struct TypeFunctionIntersectionType {
    pub(crate) components:
        alloc::vec::Vec<crate::type_aliases::type_function_type_id::TypeFunctionTypeId>,
}
