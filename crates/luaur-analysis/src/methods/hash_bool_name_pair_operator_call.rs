use crate::records::hash_bool_name_pair::HashBoolNamePair;
use crate::type_aliases::name_type_infer::Name;
use core::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

impl HashBoolNamePair {
    pub fn operator_call(&self, pair: &(bool, Name)) -> usize {
        let mut s = DefaultHasher::new();
        pair.0.hash(&mut s);
        let h1 = s.finish();

        let mut s = DefaultHasher::new();
        pair.1.hash(&mut s);
        let h2 = s.finish();

        (h1 as usize) ^ (h2 as usize)
    }
}
