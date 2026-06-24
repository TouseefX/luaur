use crate::records::block::Block;

impl Block {
    pub fn add_successor(&mut self, target: *mut Block) {
        // C++: successors.emplace_back(target); target->predecessors.emplace_back(this);
        // BlockId = NotNull<Block> = *mut Block.
        self.successors.push(target);
        unsafe { &mut *target }
            .predecessors
            .push(self as *mut Block);
    }
}
