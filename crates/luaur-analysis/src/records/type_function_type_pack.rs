use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct TypeFunctionTypePack {
    pub(crate) head: Vec<TypeFunctionTypeId>,
    pub(crate) tail: Option<TypeFunctionTypePackId>,
}
