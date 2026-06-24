use crate::records::type_pair_hash::TypePairHash;
use crate::type_aliases::type_id::TypeId;

#[allow(non_camel_case_types)]
pub type SeenSet =
    luaur_common::records::dense_hash_map::DenseHashMap<(TypeId, TypeId), bool, TypePairHash>;
