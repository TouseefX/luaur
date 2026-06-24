use crate::records::generic_counter::GenericCounter;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl GenericCounter {
    pub fn generic_counter(_cached_types: *mut DenseHashSet<TypeId>) -> Self {
        let cached_types_ptr = _cached_types;
        // GenericCounter::generic_counter expects a NotNull<DenseHashSet<TypeId>> at the Rust level,
        // but the method signature in this translation item is a raw pointer.
        // Call the existing wrapper.
        Self::generic_counter(cached_types_ptr)
    }
}
