use crate::records::block::Block;
use crate::type_aliases::instr_id::InstrId;
use alloc::vec::Vec;

pub fn block_get_instructions(block: &Block) -> &Vec<InstrId> {
    &block.instructions
}
