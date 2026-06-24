use crate::records::generic_pack_mapping::GenericPackMapping;
use crate::records::path_hash::PathHash;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

impl PathHash {
    pub fn operator_call_2(&self, mapping: &GenericPackMapping) -> usize {
        let mapped_type = mapping.mappedType;
        let mut s = DefaultHasher::new();
        mapped_type.hash(&mut s);
        s.finish() as usize
    }
}
