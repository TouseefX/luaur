use crate::functions::paged_unfreeze::paged_unfreeze;
use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
    pub fn unfreeze(&mut self) {
        for &block in &self.stuff {
            paged_unfreeze(block as *mut core::ffi::c_void, Self::kBlockSizeBytes);
        }
        self.frozen = false;
    }
}
