use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct ToStringNameMap {
    pub types: DenseHashMap<TypeId, String>,
    pub type_packs: DenseHashMap<TypePackId, String>,
}
