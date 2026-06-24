use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone, Default)]
pub struct InteriorFreeTypes {
    pub types: alloc::vec::Vec<TypeId>,
    pub type_packs: alloc::vec::Vec<TypePackId>,
}
