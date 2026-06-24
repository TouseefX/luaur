//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:29:set_set`
//! Source: `Analysis/include/Luau/Set.h:29-32` (hand-ported)

use crate::records::set::Set;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl<T: Clone + core::hash::Hash + PartialEq> Set<T> {
    /// C++ `explicit Set(const T& empty_key)`.
    pub fn new(empty_key: T) -> Self {
        Self {
            mapping: DenseHashMap::new(empty_key),
            entry_count: 0,
        }
    }
}
