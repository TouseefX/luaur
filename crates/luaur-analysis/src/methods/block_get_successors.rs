//! Source: `Analysis/src/ControlFlowGraph.cpp:97-100` (hand-ported)
//! C++ `const std::vector<BlockId>& Block::getSuccessors() const`.
use crate::records::block::Block;
use crate::type_aliases::block_id::BlockId;
use alloc::vec::Vec;

/// `const std::vector<BlockId>& Block::getSuccessors() const`.
pub fn block_get_successors(block: &Block) -> &Vec<BlockId> {
    // C++: return successors;
    block.get_successors()
}
