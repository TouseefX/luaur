use crate::type_aliases::type_pack_id::TypePackId;

pub type SeenTypePacks = luaur_common::records::dense_hash_map::DenseHashMap<TypePackId, TypePackId>;
