use crate::type_aliases::type_id::TypeId;

pub type SeenTypes = luaur_common::records::dense_hash_map::DenseHashMap<TypeId, TypeId>;
