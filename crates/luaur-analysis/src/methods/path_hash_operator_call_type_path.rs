use crate::records::path_hash::PathHash;
use crate::records::property_type_path::Property;
use core::hash::{Hash, Hasher};

impl PathHash {
    pub fn operator_call_7(&self, prop: &Property) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        prop.name().hash(&mut hasher);
        let hash_name = hasher.finish() as usize;

        let hash_read = if prop.isRead() { 1usize } else { 0usize };

        hash_name ^ hash_read
    }
}
