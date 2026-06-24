//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:88:set_empty`
//! Source: `Analysis/include/Luau/Set.h:88-91` (hand-ported)

use crate::records::set::Set;

impl<T: Clone + core::hash::Hash + PartialEq> Set<T> {
    pub fn empty(&self) -> bool {
        self.entry_count == 0
    }
}
