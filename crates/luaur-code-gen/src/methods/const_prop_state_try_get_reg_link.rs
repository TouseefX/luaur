use crate::enums::ir_op_kind::IrOpKind;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_op::IrOp;
use crate::records::register_link::RegisterLink;

impl ConstPropState {
    pub fn try_get_reg_link(&mut self, inst_op: IrOp) -> Option<*mut RegisterLink> {
        if inst_op.kind() != IrOpKind::Inst {
            return None;
        }
        if let Some(link) = self.inst_link.find(&inst_op.index()) {
            if link.version < self.regs[link.reg as usize].version {
                return None;
            }
            return Some(link as *const RegisterLink as *mut RegisterLink);
        }
        None
    }
}
