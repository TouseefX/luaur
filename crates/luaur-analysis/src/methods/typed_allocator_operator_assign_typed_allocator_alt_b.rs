use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
    #[allow(non_snake_case)]
    pub fn operator_assign_mut(&mut self, other: TypedAllocator<T>) -> &mut TypedAllocator<T> {
        *self = other;
        self
    }
}
