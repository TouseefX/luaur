use crate::records::type_id_pair_hash::TypeIdPairHash;
use crate::type_aliases::type_id::TypeId;

pub type SeenTablePropPairs =
    luaur_common::records::dense_hash_map::DenseHashMap<(TypeId, TypeId), bool, TypeIdPairHash>;
