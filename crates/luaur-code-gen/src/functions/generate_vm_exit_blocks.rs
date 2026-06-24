use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::add_use::add_use;
use crate::functions::has_side_effects::has_side_effects;
use crate::functions::is_unsafe_to_sink::is_unsafe_to_sink;
use crate::functions::remove_use::remove_use;
use crate::functions::replace_ir_utils::replace_ir_function_ir_op_ir_op;
use crate::functions::visit_arguments::visit_arguments;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::records::vm_exit_sync_info::VmExitSyncInfo;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_map::DenseHashMap;

fn collect_inputs(inst: &mut IrInst, inputs: &mut Vec<(IrOp, u32)>) {
    visit_arguments(inst, |op| {
        if op.kind() == IrOpKind::Inst {
            if let Some(slot) = inputs.iter_mut().find(|el| el.0 == op) {
                slot.1 += 1;
            } else {
                inputs.push((op, 1));
            }
        }
    });
}

fn redirect(op: &mut IrOp, inst_redir: &DenseHashMap<u32, u32>, inputs: &[(IrOp, u32)]) {
    if op.kind() == IrOpKind::Inst {
        if let Some(new_index) = inst_redir.find(&op.index()) {
            *op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, *new_index);
        } else if !inputs.iter().any(|el| el.0 == *op) {
            // Values can only be used if they are defined in the same block or be an input
            CODEGEN_ASSERT!(false);
        }
    }
}

pub fn generate_vm_exit_blocks(build: &mut IrBuilder, recorded_vm_exit_syncs: &Vec<u32>) {
    let function: *mut IrFunction = &mut build.function;

    for &vm_exit_sync_location in recorded_vm_exit_syncs {
        let sync_info: *mut VmExitSyncInfo = unsafe {
            (*function)
                .vm_exit_info
                .get_or_insert(vm_exit_sync_location)
        };

        if unsafe { (*sync_info).reg_stores.is_empty() } {
            continue;
        }

        // We will be collecting instructions we want to move into the VM exit in reverse order
        let mut store_instructions: Vec<IrInst> = Vec::new();
        let mut arg_instructions: Vec<u32> = Vec::new();
        let mut inputs: Vec<(IrOp, u32)> = Vec::new();

        // Start with the store instruction we got
        let n_reg = unsafe { (*sync_info).reg_stores.len() };
        for ri in 0..n_reg {
            let n_st = unsafe { (&(*sync_info).reg_stores)[ri].stores.size() as usize };
            for si in 0..n_st {
                let mut backup = unsafe {
                    (&(*sync_info).reg_stores)[ri].stores.as_slice()[si]
                        .backup
                        .clone()
                };
                collect_inputs(&mut backup, &mut inputs);
                store_instructions.push(backup);
            }
        }

        // For each input we got, see if we are the only user of it and if we are (and it has no side effects), schedule a move inside
        let mut i = 0usize;
        while i < inputs.len() {
            let input_op = inputs[i].0;
            let input_count = inputs[i].1;

            let inst_use_count = unsafe { (*function).inst_op(input_op).use_count };
            let inst_cmd = unsafe { (*function).inst_op(input_op).cmd };

            if (inst_use_count as u32) == input_count
                && !has_side_effects(inst_cmd)
                && !is_unsafe_to_sink(inst_cmd)
            {
                let inst_idx = input_op.index();
                arg_instructions.push(inst_idx);

                inputs.remove(i); // Delete this input

                let inst_ptr: *mut IrInst =
                    unsafe { &mut (&mut (*function).instructions)[inst_idx as usize] };
                collect_inputs(unsafe { &mut *inst_ptr }, &mut inputs);
            } else {
                i += 1;
            }
        }

        for input in &inputs {
            unsafe {
                (*sync_info).arg_ops.push_back(input.0);
            }
        }

        // We now should have an extracted instruction chain with no side effects in reverse order
        let block_op = build.block(IrBlockKind::ExitSync);
        unsafe {
            (*sync_info).block = block_op;
            *(*function)
                .block_to_vm_exit_map
                .get_or_insert(block_op.index()) = vm_exit_sync_location;
        }
        build.begin_block(block_op);

        let mut inst_redir: DenseHashMap<u32, u32> = DenseHashMap::new(!0u32);

        for k in (0..arg_instructions.len()).rev() {
            let inst_idx = arg_instructions[k];

            CODEGEN_ASSERT!((inst_idx as usize) < unsafe { (*function).instructions.len() });
            let mut clone = unsafe { (&(*function).instructions)[inst_idx as usize].clone() };

            let cn = clone.ops.size();
            for oi in 0..cn {
                redirect(
                    &mut clone.ops.as_mut_slice()[oi as usize],
                    &inst_redir,
                    &inputs,
                );
            }

            for oi in 0..cn {
                let op = clone.ops.as_slice()[oi as usize];
                add_use(unsafe { &mut *function }, op);
            }

            // Instructions that referenced the original will have to be adjusted to use the clone
            unsafe {
                *inst_redir.get_or_insert(inst_idx) = (*function).instructions.len() as u32;
            }

            // Reconstruct the fresh clone
            build.inst_ir_cmd_ir_ops(clone.cmd, &clone.ops);
        }

        for si in 0..store_instructions.len() {
            let mut clone = store_instructions[si].clone();

            let cn = clone.ops.size();
            for oi in 0..cn {
                redirect(
                    &mut clone.ops.as_mut_slice()[oi as usize],
                    &inst_redir,
                    &inputs,
                );
            }

            for oi in 0..cn {
                let op = clone.ops.as_slice()[oi as usize];
                add_use(unsafe { &mut *function }, op);
            }

            // Reconstruct the fresh clone
            build.inst_ir_cmd_ir_ops(clone.cmd, &clone.ops);

            let mut orig = store_instructions[si].clone();
            visit_arguments(&mut orig, |op| {
                remove_use(unsafe { &mut *function }, op);
            });
        }

        let vm_exit = unsafe { (*sync_info).vm_exit };
        build.inst_ir_cmd_ir_op(IrCmd::JUMP, vm_exit);

        // Replace guard VM exit with an exit sync block
        let block_for_replace = unsafe { (*sync_info).block };
        let guard_op_count = unsafe {
            (&(*function).instructions)[vm_exit_sync_location as usize]
                .ops
                .size()
        };
        for oi in 0..guard_op_count {
            let op = unsafe {
                (&(*function).instructions)[vm_exit_sync_location as usize]
                    .ops
                    .as_slice()[oi as usize]
            };
            if op.kind() == IrOpKind::VmExit && op == vm_exit {
                let op_ptr: *mut IrOp = unsafe {
                    &mut (&mut (*function).instructions)[vm_exit_sync_location as usize]
                        .ops
                        .as_mut_slice()[oi as usize]
                };
                replace_ir_function_ir_op_ir_op(
                    unsafe { &mut *function },
                    unsafe { &mut *op_ptr },
                    block_for_replace,
                );
                break;
            }
        }
    }
}
