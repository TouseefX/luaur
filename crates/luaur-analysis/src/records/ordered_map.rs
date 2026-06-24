use luaur_common::records::dense_hash_map::DenseHashMap;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct OrderedMap<K, V>
where
    K: Clone + core::hash::Hash + Eq,
    V: Clone + luaur_common::records::dense_hash_table::DenseDefault,
{
    pub(crate) keys: alloc::vec::Vec<K>,
    pub(crate) pairings: DenseHashMap<K, V>,
}
