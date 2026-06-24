use crate::type_aliases::type_id::TypeId;

pub fn type_id_pair_hash_hash_one(key: TypeId) -> usize {
    ((key as usize) >> 4) ^ ((key as usize) >> 9)
}
