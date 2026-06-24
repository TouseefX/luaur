//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Set.h:48:set_insert`
//! Source: `Analysis/include/Luau/Set.h:48-53` (hand-ported)

use crate::records::set::Set;

impl<T: Clone + core::hash::Hash + PartialEq> Set<T> {
    /// C++ `template<class Iterator> void insert(Iterator begin, Iterator end)`.
    pub fn insert_range<'a, I: Iterator<Item = &'a T>>(&mut self, items: I)
    where
        T: 'a,
    {
        for it in items {
            self.insert(it);
        }
    }
}
