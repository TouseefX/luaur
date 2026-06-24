//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:99:set_contains`
//! Source: `Analysis/include/Luau/Set.h:99-102` (hand-ported)

use crate::records::set::Set;

impl<T: Clone + core::hash::Hash + PartialEq> Set<T> {
    pub fn contains(&self, element: &T) -> bool {
        self.count(element) != 0
    }
}
