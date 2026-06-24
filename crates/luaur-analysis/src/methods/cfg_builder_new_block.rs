//! Source: `Analysis/src/ControlFlowGraph.cpp:245-251` (hand-ported)
//! C++ `Block* CFGBuilder::newBlock(BlockKind kind, std::string debugName, Block* pred)`.
use crate::enums::block_kind::BlockKind;
use crate::records::block::Block;
use crate::records::cfg_builder::CfgBuilder;
use alloc::string::String;

impl CfgBuilder {
    /// C++ default arg `Block* pred = nullptr`; callers pass `null_mut()` to omit.
    pub fn new_block(
        &mut self,
        kind: BlockKind,
        debug_name: String,
        pred: *mut Block,
    ) -> *mut Block {
        // C++:
        //   Block* b = cfg->newBlock(kind, debugName);
        //   if (pred) pred->addSuccessor(b);
        //   return b;
        let b = self.cfg.as_mut().unwrap().new_block(kind, debug_name);
        if !pred.is_null() {
            unsafe { (*pred).add_successor(b) };
        }
        b
    }
}
