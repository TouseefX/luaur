pub type Bindings = luaur_common::records::dense_hash_map::DenseHashMap<
    crate::records::symbol::Symbol,
    *const crate::records::def::Def,
>;
