use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_block::{kBlockNoStartPc, IrBlock};
use crate::records::ir_builder::IrBuilder;

impl IrBuilder {
    pub fn check_safe_env(&mut self, pcpos: i32) {
        let active_block_idx = self.active_block_idx as usize;
        let active: &mut IrBlock = &mut self.function.blocks[active_block_idx];

        const k_block_flag_safe_env_check: u8 = 1 << 0;
        const k_block_flag_safe_env_clear: u8 = 1 << 1;

        if active.startpc != kBlockNoStartPc {
            if (active.flags & k_block_flag_safe_env_clear) == 0 {
                active.flags |= k_block_flag_safe_env_check;
            }
        }

        let exit_op = self.vm_exit(pcpos as u32);
        self.inst_ir_cmd_ir_op(IrCmd::CHECK_SAFE_ENV, exit_op);
    }
}
