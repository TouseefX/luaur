use crate::functions::paged_allocate::paged_allocate;
use crate::records::typed_allocator::TypedAllocator;

impl<T> TypedAllocator<T> {
    pub(crate) fn append_block(&mut self) {
        let block = paged_allocate(Self::kBlockSizeBytes);
        if block.is_null() {
            panic!("std::bad_alloc");
        }

        self.stuff.push(block as *mut T);
        self.current_block_size = 0;
    }
}
