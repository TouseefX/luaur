use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper {
    pub fn size(&self) -> usize {
        (self.itEnd as usize).wrapping_sub(self.itBegin as usize) / core::mem::size_of::<u32>()
    }
}
