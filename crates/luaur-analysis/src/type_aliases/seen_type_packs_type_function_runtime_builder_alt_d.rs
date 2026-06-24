use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub type SeenTypePacks = DenseHashMap<TypeFunctionTypePackId, TypePackId>;
