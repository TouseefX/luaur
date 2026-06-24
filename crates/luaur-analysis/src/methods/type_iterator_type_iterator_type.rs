//! Node: `cxx:Method:Luau.Analysis:Analysis/include/Luau/Type.h:1138:type_iterator_type_iterator`
//! Source: `Analysis/include/Luau/Type.h:1138-1148` (hand-ported)

use crate::records::type_iterator::{TypeIterator, TypeIteratorMember};
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl<T: TypeIteratorMember> TypeIterator<T> {
    /// C++ `explicit TypeIterator(const T* t)`.
    pub fn type_iterator_type(t: *const T) -> Self {
        LUAU_ASSERT!(!t.is_null());

        let mut it = Self::type_iterator_default();

        unsafe {
            let types = (*t).get_types();
            if !types.is_empty() {
                it.stack.push_front((t, 0));
            }

            it.seen.insert(t);
            it.descend();
        }

        it
    }
}
