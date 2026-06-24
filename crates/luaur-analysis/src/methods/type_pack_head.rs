use crate::records::type_pack::TypePack;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;

impl TypePack {
    pub fn head(&self) -> &Vec<TypeId> {
        &self.head
    }
}
