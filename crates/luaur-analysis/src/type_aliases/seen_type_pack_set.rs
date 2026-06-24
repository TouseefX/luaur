use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

pub type SeenTypePackSet = DenseHashMap<(TypePackId, TypePackId), bool, TypePairHash>;
