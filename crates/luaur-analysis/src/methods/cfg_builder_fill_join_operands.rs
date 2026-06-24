//! Source: `Analysis/src/ControlFlowGraph.cpp:488-497` (hand-ported)
//! C++ `void CFGBuilder::fillJoinOperands(Block* block, Join* j)`.
use crate::records::block::Block;
use crate::records::cfg_builder::CfgBuilder;
use crate::records::join::Join;
use crate::type_aliases::block_id::BlockId;

impl CfgBuilder {
    pub fn fill_join_operands(&mut self, block: *mut Block, j: *mut Join) {
        unsafe {
            // C++:
            //   for (BlockId pred : block->getPredecessors()) {
            //       auto def = readVariable(pred, j->definition->sym);
            //       j->operands.emplace_back(def);
            //   }
            //   trimTrivialJoin(j);
            // Snapshot predecessors: readVariable recurses and may mutate `block`.
            let preds: alloc::vec::Vec<BlockId> = (*block).get_predecessors().clone();
            let sym = (*(*j).definition).sym.clone();
            for pred in preds {
                let def = self.read_variable(pred, sym.clone());
                (*j).operands.push(def);
            }
            self.trim_trivial_join(j);
        }
    }
}
