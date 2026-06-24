use crate::records::block_iterator_wrapper::BlockIteratorWrapper;

impl BlockIteratorWrapper {
    pub fn empty(&self) -> bool {
        self.itBegin == self.itEnd
    }
}
