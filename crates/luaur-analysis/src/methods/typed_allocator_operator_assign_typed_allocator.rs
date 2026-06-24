use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
    /// @delete
    #[allow(dead_code)]
    pub fn operator_assign(&mut self, _other: &TypedAllocator<T>) -> &mut TypedAllocator<T> {
        unimplemented!("TypedAllocator copy assignment is deleted in C++")
    }
}
