#[derive(Debug, Clone)]
pub struct SkipCacheForType {
    pub skip_cache_for_type: *const luaur_common::records::dense_hash_map::DenseHashMap<
        crate::type_aliases::type_id::TypeId,
        bool,
    >,
    pub type_arena: *const crate::records::type_arena::TypeArena,
    pub result: bool,
}
