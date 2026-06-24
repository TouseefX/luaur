use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
    /// @delete
    #[allow(dead_code)]
    pub fn typed_allocator_typed_allocator(_other: &TypedAllocator<T>) {
        unimplemented!("TypedAllocator copy constructor is deleted in C++")
    }
}
