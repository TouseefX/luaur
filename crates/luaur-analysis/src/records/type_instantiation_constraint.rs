use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct TypeInstantiationConstraint {
    pub(crate) function_type: TypeId,
    pub(crate) placeholder_type: TypeId,
    pub(crate) type_arguments: Vec<TypeId>,
    pub(crate) type_pack_arguments: Vec<TypePackId>,
}
