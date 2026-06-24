use crate::enums::condition_x_64::ConditionX64;
use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::get_negated_condition_condition_x_64::get_negated_condition;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_block::IrBlock;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;
use luaur_common::FFlag;

impl IrLoweringX64 {
    pub fn jump_or_abort_on_undef_no_finalize(
        &mut self,
        cond: ConditionX64,
        target: IrOp,
        index: u32,
        next: &IrBlock,
        fresh: &mut Label,
    ) {
        CODEGEN_ASSERT!(FFlag::LuauCodegenVmExitSync.get());

        if target.kind() == IrOpKind::Block && self.block_op(target).kind == IrBlockKind::ExitSync {
            let token = self.regs.get_alloc_token();

            if self.exit_sync_inst_idx != index {
                self.exit_sync_inst_idx = index;
                self.exit_sync_alloc_token = token;
            } else {
                CODEGEN_ASSERT!(self.exit_sync_alloc_token == token);
            }
        }

        let label = self.get_target_label(target, index, fresh) as *mut Label;

        unsafe {
            if target.kind() == IrOpKind::Undef {
                if cond == ConditionX64::Count {
                    (*self.build).ud_2();
                } else {
                    (*self.build).jcc(get_negated_condition(cond), &mut *label);
                    (*self.build).ud_2();
                    (*self.build).set_label(&mut *label);
                }
            } else if cond == ConditionX64::Count {
                let should_jump = if target.kind() == IrOpKind::VmExit {
                    true
                } else {
                    let target_block = self.block_op(target) as *mut IrBlock;
                    !self.is_fallthrough_block(&*target_block, next)
                };

                if should_jump {
                    (*self.build).jmp_label(&mut *label);
                }
            } else {
                (*self.build).jcc(cond, &mut *label);
            }
        }
    }
}
