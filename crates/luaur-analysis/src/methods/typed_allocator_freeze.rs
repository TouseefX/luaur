use crate::functions::paged_freeze::paged_freeze;
use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
    pub fn freeze(&mut self) {
        for &block in &self.stuff {
            paged_freeze(block as *mut core::ffi::c_void, Self::kBlockSizeBytes);
        }
        self.frozen = true;
    }
}
