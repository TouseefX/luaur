use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::visit_arguments::visit_arguments;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_inst::IrInst;
use crate::records::remove_dead_store_state::RemoveDeadStoreState;
use alloc::vec::Vec;

pub fn update_remaining_uses(state: &mut RemoveDeadStoreState, inst: &mut IrInst, index: u32) {
    let remaining: *mut Vec<u32> = state.remaining_uses;

    unsafe {
        (&mut *remaining)[index as usize] = inst.use_count as u32;
    }

    visit_arguments(inst, |op| {
        if op.kind() == IrOpKind::Inst {
            unsafe {
                CODEGEN_ASSERT!((&*remaining)[op.index() as usize] != 0);
                (&mut *remaining)[op.index() as usize] -= 1;
            }
        }
    });
}
