use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::can_invalidate_safe_env::can_invalidate_safe_env;
use crate::functions::is_block_terminator::is_block_terminator;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use crate::type_aliases::ir_ops::IrOps;

const K_BLOCK_FLAG_SAFE_ENV_CLEAR: u8 = 1 << 1;

impl IrBuilder {
    pub fn inst_ir_cmd_ir_ops(&mut self, cmd: IrCmd, ops: &IrOps) -> IrOp {
        let index = self.function.instructions.len() as u32;
        self.function
            .instructions
            .push(crate::records::ir_inst::IrInst {
                cmd,
                ops: ops.clone(),
                last_use: 0,
                use_count: 0,
                reg_x64: crate::records::register_x_64::RegisterX64::default(),
                reg_a64: crate::records::register_a_64::RegisterA64::default(),
                reused_reg: false,
                spilled: false,
                needs_reload: false,
            });

        CODEGEN_ASSERT!(!self.in_terminated_block);

        if is_block_terminator(cmd) {
            self.function.blocks[self.active_block_idx as usize].finish = index;
            self.in_terminated_block = true;
        }

        if can_invalidate_safe_env(cmd) {
            self.function.blocks[self.active_block_idx as usize].flags |=
                K_BLOCK_FLAG_SAFE_ENV_CLEAR;
        }

        IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, index)
    }
}
