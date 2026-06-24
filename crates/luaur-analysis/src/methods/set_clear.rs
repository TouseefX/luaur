//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:77:set_clear`
//! Source: `Analysis/include/Luau/Set.h:77-81` (hand-ported)

use crate::records::set::Set;

impl<T: Clone + core::hash::Hash + PartialEq> Set<T> {
    pub fn clear(&mut self) {
        self.mapping.clear();
        self.entry_count = 0;
    }
}
