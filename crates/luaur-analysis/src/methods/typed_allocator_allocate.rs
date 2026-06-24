use crate::records::typed_allocator::TypedAllocator;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl<T> TypedAllocator<T> {
    pub fn allocate(&mut self, value: T) -> *mut T {
        LUAU_ASSERT!(!self.frozen);

        if self.current_block_size >= Self::kBlockSize {
            LUAU_ASSERT!(self.current_block_size == Self::kBlockSize);
            self.append_block();
        }

        let block = *self.stuff.last().unwrap();
        let res = unsafe { block.add(self.current_block_size) };
        unsafe {
            core::ptr::write(res, value);
        }
        self.current_block_size += 1;
        res
    }
}
