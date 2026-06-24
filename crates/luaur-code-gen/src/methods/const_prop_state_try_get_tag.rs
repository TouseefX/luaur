use crate::enums::ir_op_kind::IrOpKind;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_op::IrOp;

impl ConstPropState {
    pub fn try_get_tag(&mut self, op: IrOp) -> u8 {
        if let Some(info) = self.try_get_register_info(op) {
            unsafe {
                if (*info).tag != 0xff {
                    return (*info).tag;
                }
            }
        }
        if op.kind() == IrOpKind::Inst {
            if let Some(info) = self.inst_tag.find(&op.index()) {
                return *info;
            }
        }
        0xff
    }
}
