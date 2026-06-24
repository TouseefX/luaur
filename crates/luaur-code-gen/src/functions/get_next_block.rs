use crate::enums::ir_block_kind::IrBlockKind;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;

pub fn get_next_block<'a>(
    function: &'a mut IrFunction,
    sorted_blocks: &[u32],
    dummy: &'a mut IrBlock,
    i: usize,
) -> &'a mut IrBlock {
    for j in (i + 1)..sorted_blocks.len() {
        let block_idx = sorted_blocks[j] as usize;
        if function.blocks[block_idx].kind != IrBlockKind::Dead {
            return &mut function.blocks[block_idx];
        }
    }

    dummy
}
