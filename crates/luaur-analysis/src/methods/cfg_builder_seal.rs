use crate::records::block::Block;
use crate::records::cfg_builder::CfgBuilder;

impl CfgBuilder {
    /// `void CFGBuilder::seal(Block* b)`. Reference: `ControlFlowGraph.cpp`.
    pub fn seal(&mut self, b: *mut Block) {
        unsafe {
            // C++:
            //   auto joinsToFill = incompleteJoins.find(b);
            //   if (joinsToFill != nullptr)
            //       for (auto j : *joinsToFill) fillJoinOperands(b, j);
            //   sealedBlocks.insert(b);
            if let Some(joins) = self.incomplete_joins.find(&b) {
                let joins: alloc::vec::Vec<_> = joins.iter().copied().collect();
                for j in joins {
                    self.fill_join_operands(b, j);
                }
            }
            self.sealed_blocks.insert(b);
        }
    }
}
