//! Source: `Analysis/src/ControlFlowGraph.cpp:92-95` (hand-ported)
//! C++ `const std::vector<BlockId>& Block::getPredecessors() const`.
use crate::records::block::Block;
use crate::type_aliases::block_id::BlockId;
use alloc::vec::Vec;

/// `const std::vector<BlockId>& Block::getPredecessors() const`.
pub fn block_get_predecessors(block: &Block) -> &Vec<BlockId> {
    // C++: return predecessors;
    block.get_predecessors()
}
