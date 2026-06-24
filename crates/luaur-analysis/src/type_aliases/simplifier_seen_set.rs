use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::type_id::TypeId;

pub type SimplifierSeenSet =
    luaur_common::records::dense_hash_map::DenseHashMap<(TypeId, TypeId), bool, TypePairHash>;
