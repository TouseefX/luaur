use crate::functions::propagate_tags_from_predecessors::propagate_tags_from_predecessors;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use crate::records::store_reg_info::StoreRegInfo;
use alloc::boxed::Box;

pub fn setup_block_entry_state_ir_function_ir_block_remove_dead_store_state(
    function: &IrFunction,
    block: &IrBlock,
    state: &mut RemoveDeadStoreState,
) {
    let info_ptr: *mut [StoreRegInfo; 256] = &mut state.info;

    let get: Box<dyn Fn(usize) -> u8> = {
        let p = info_ptr;
        Box::new(move |i: usize| -> u8 { unsafe { (*p)[i].known_tag } })
    };

    let set: Box<dyn Fn(usize, u8)> = {
        let p = info_ptr;
        Box::new(move |i: usize, tag: u8| unsafe {
            (*p)[i].known_tag = tag;
        })
    };

    propagate_tags_from_predecessors(function, block, get, set);
}
