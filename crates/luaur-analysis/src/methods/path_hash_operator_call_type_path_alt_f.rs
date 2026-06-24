use crate::records::path_hash::PathHash;
use crate::records::reduction::Reduction;
use crate::type_aliases::type_id::TypeId;
use std::hash::{Hash, Hasher};

impl PathHash {
    pub fn operator_call_8(&self, reduction: &Reduction) -> usize {
        let ty: TypeId = reduction.resultType;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ty.hash(&mut hasher);
        hasher.finish() as usize
    }
}
