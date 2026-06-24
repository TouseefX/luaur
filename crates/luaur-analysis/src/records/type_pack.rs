use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct TypePack {
    pub(crate) head: alloc::vec::Vec<TypeId>,
    pub(crate) tail: Option<TypePackId>,
}

impl TypePack {
    pub fn new(head: alloc::vec::Vec<TypeId>, tail: Option<TypePackId>) -> Self {
        Self { head, tail }
    }
}
