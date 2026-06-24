use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::const_prop_state::ConstPropState;
use crate::records::register_link::RegisterLink;

impl ConstPropState {
    pub fn create_reg_link(&mut self, inst_idx: u32, reg_op: crate::records::ir_op::IrOp) {
        CODEGEN_ASSERT!(!self.inst_link.contains(&inst_idx));
        let reg = vm_reg_op(reg_op) as u8;
        let version = self.regs[reg as usize].version;
        self.inst_link
            .try_insert(inst_idx, RegisterLink { reg, version });
    }
}
