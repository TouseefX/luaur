use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper {
    pub fn operator_index(&self, pos: usize) -> u32 {
        CODEGEN_ASSERT!(
            pos < (self.itEnd as usize).wrapping_sub(self.itBegin as usize)
                / core::mem::size_of::<u32>()
        );
        unsafe { *self.itBegin.add(pos) }
    }
}
