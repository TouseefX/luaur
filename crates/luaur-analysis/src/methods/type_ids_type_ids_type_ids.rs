use crate::records::type_ids::TypeIds;

impl TypeIds {
    pub fn type_ids() -> Self {
        Self {
            types: luaur_common::records::dense_hash_map::DenseHashMap::new(core::ptr::null()),
            order: Vec::new(),
            hash: 0,
        }
    }
}
