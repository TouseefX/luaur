//! Source: `Analysis/src/ControlFlowGraph.cpp:102-105` (hand-ported)
//! C++ `Block* CFGAllocator::newBlock(BlockKind kind, std::string debugName)`.
use crate::enums::block_kind::BlockKind;
use crate::records::block::Block;
use crate::records::cfg_allocator::CfgAllocator;
use alloc::string::String;

impl CfgAllocator {
    pub fn new_block(&mut self, kind: BlockKind, debug_name: String) -> *mut Block {
        // C++: return block.allocate(kind, debugName);
        // TypedAllocator::allocate takes the constructed value; build the Block
        // in place (C++ constructs `T{args...}` inside allocate).
        self.block.allocate(Block::new(kind, debug_name))
    }
}
