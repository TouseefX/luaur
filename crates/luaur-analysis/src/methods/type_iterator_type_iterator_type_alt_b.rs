//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1201:type_iterator_type_iterator`
//! Source: `Analysis/include/Luau/Type.h:1201` (hand-ported)

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;

impl<T: TypeIteratorMember> TypeIterator<T> {
    /// C++ private `TypeIterator() = default;` — the `end()` sentinel.
    pub fn type_iterator_default() -> Self {
        Self {
            stack: VecDeque::new(),
            seen: DenseHashSet::new(core::ptr::null()),
        }
    }
}
