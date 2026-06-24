use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;

pub fn save_block_exit_state(
    function: &mut IrFunction,
    block: &IrBlock,
    state: &mut ConstPropState,
) {
    let mut tags: alloc::vec::Vec<u8> =
        alloc::vec::Vec::with_capacity((state.max_reg as usize) + 1);

    for i in 0..=(state.max_reg as usize) {
        tags.push(state.regs[i].tag);
    }

    let block_idx = function.get_block_index(block);
    function.block_exit_tags[block_idx as usize] = tags;
}
