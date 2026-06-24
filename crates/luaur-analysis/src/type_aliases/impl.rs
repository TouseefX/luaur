use luaur_common::records::dense_hash_map::DenseHashMap;

#[allow(non_camel_case_types)]
pub type Impl<T, Hash> = DenseHashMap<T, bool, Hash>;
