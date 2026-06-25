use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::kill_ir_utils::kill_ir_function_ir_inst;
use crate::functions::mark_dead_stores_in_block::mark_dead_stores_in_block;
use crate::functions::setup_block_entry_state_optimize_dead_store::setup_block_entry_state_ir_function_ir_block_remove_dead_store_state;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_block::IrBlock;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use alloc::vec::Vec;

pub fn mark_dead_stores_in_block_chain(
    build: &mut IrBuilder,
    visited: &mut Vec<u8>,
    remaining_uses: &mut Vec<u32>,
    block_idx_chain: &mut Vec<u32>,
    all_recorded_vm_exit_syncs: &mut Vec<u32>,
    start_block_idx: u32,
) {
    let function: *mut IrFunction = &mut build.function;

    let mut state = RemoveDeadStoreState::remove_dead_store_state_remove_dead_store_state(
        unsafe { &mut *function },
        remaining_uses,
    );

    // We will be visiting this chain a few times to clean unreferenced temporaries
    // Clear the storage we reuse
    block_idx_chain.clear();

    {
        let block_ptr: *const IrBlock = unsafe { &(&(*function).blocks)[start_block_idx as usize] };
        setup_block_entry_state_ir_function_ir_block_remove_dead_store_state(
            unsafe { &*function },
            unsafe { &*block_ptr },
            &mut state,
        );
    }

    let mut cur: i64 = start_block_idx as i64;

    while cur >= 0 {
        let block_idx = cur as u32;

        CODEGEN_ASSERT!(visited[block_idx as usize] == 0);
        visited[block_idx as usize] = 1;

        block_idx_chain.push(block_idx);

        {
            let block_ptr: *mut IrBlock =
                unsafe { &mut (&mut (*function).blocks)[block_idx as usize] };
            mark_dead_stores_in_block(build, unsafe { &mut *block_ptr }, &mut state);
        }

        let finish = unsafe { (&(*function).blocks)[block_idx as usize].finish };
        let term_cmd = unsafe { (&(*function).instructions)[finish as usize].cmd };
        let term_op_a = unsafe { (&(*function).instructions)[finish as usize].ops[0] };

        let mut next: i64 = -1;

        // Unconditional jump into a block with a single user (current block) allows us to continue optimization
        if term_cmd == IrCmd::JUMP && term_op_a.kind() == IrOpKind::Block {
            let target_idx = term_op_a.index();
            let target_use_count = unsafe { (&(*function).blocks)[target_idx as usize].use_count };
            let target_kind = unsafe { (&(*function).blocks)[target_idx as usize].kind };

            if target_use_count == 1
                && visited[target_idx as usize] == 0
                && target_kind != IrBlockKind::Fallback
            {
                // If this block isn't glued to the target in the lowering order, we cannot capture any remaining stores from it in ExitSync blocks
                let expected_next =
                    unsafe { (&(*function).blocks)[block_idx as usize].expected_next_block };
                if luaur_common::FFlag::LuauCodegenVmExitSyncFix.get()
                    && expected_next != target_idx
                {
                    state.invalidate_value_propagation();
                }

                next = target_idx as i64;
            }
        }

        cur = next;
    }

    if luaur_common::FFlag::LuauCodegenVmExitSync.get() {
        state.prune_vm_exit_info();

        all_recorded_vm_exit_syncs.extend_from_slice(&state.recorded_vm_exit_syncs);
    }

    // If there are allocating instructions, check if they have 'read' uses after DSE
    if state.has_allocations {
        let mut found_unused = false;

        // Remove uses in instructions writing to the allocations
        for ci in 0..block_idx_chain.len() {
            let b_idx = block_idx_chain[ci];
            let bstart = unsafe { (&(*function).blocks)[b_idx as usize].start };
            let bfinish = unsafe { (&(*function).blocks)[b_idx as usize].finish };

            let mut index = bstart;
            while index <= bfinish {
                let cmd = unsafe { (&(*function).instructions)[index as usize].cmd };
                let use_count = unsafe { (&(*function).instructions)[index as usize].use_count };
                unsafe {
                    (&mut *state.remaining_uses)[index as usize] = use_count as u32;
                }

                match cmd {
                    IrCmd::BUFFER_WRITEI8
                    | IrCmd::BUFFER_WRITEI16
                    | IrCmd::BUFFER_WRITEI32
                    | IrCmd::BUFFER_WRITEI64
                    | IrCmd::BUFFER_WRITEF32
                    | IrCmd::BUFFER_WRITEF64 => {
                        let op_a_index =
                            unsafe { (&(*function).instructions)[index as usize].ops[0].index() };
                        unsafe {
                            (&mut *state.remaining_uses)[op_a_index as usize] -= 1;

                            if (&*state.remaining_uses)[op_a_index as usize] == 0 {
                                found_unused = true;
                            }
                        }
                    }
                    _ => {}
                }

                index += 1;
            }
        }

        // Remove those write instructions if they were the only users of the allocation
        if found_unused {
            for ci in 0..block_idx_chain.len() {
                let b_idx = block_idx_chain[ci];
                let bstart = unsafe { (&(*function).blocks)[b_idx as usize].start };
                let bfinish = unsafe { (&(*function).blocks)[b_idx as usize].finish };

                let mut index = bstart;
                while index <= bfinish {
                    let cmd = unsafe { (&(*function).instructions)[index as usize].cmd };

                    match cmd {
                        IrCmd::BUFFER_WRITEI8
                        | IrCmd::BUFFER_WRITEI16
                        | IrCmd::BUFFER_WRITEI32
                        | IrCmd::BUFFER_WRITEI64
                        | IrCmd::BUFFER_WRITEF32
                        | IrCmd::BUFFER_WRITEF64 => {
                            let op_a =
                                unsafe { (&(*function).instructions)[index as usize].ops[0] };
                            let op_a_index = op_a.index();

                            if unsafe { (&*state.remaining_uses)[op_a_index as usize] } == 0 {
                                let pointer_cmd = unsafe { (*function).inst_op(op_a).cmd };

                                if pointer_cmd == IrCmd::NEW_USERDATA {
                                    let inst_ptr: *mut IrInst = unsafe {
                                        &mut (&mut (*function).instructions)[index as usize]
                                    };
                                    kill_ir_function_ir_inst(unsafe { &mut *function }, unsafe {
                                        &mut *inst_ptr
                                    });
                                }
                            }
                        }
                        _ => {}
                    }

                    index += 1;
                }
            }
        }
    }
}
