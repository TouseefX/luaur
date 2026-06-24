use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct UnpackConstraint {
    pub(crate) result_pack: Vec<TypeId>,
    pub(crate) source_pack: TypePackId,
}
