use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::type_aliases::ir_ops::IrOps;

impl ConstPropState {
    pub fn versioned_vm_reg_load_ir_cmd_ir_op(&mut self, load_cmd: IrCmd, mut op: IrOp) -> IrInst {
        let version = self.regs[vm_reg_op(op) as usize].version;
        op = IrOp::ir_op_ir_op_kind_u32(IrOpKind::VmReg, (vm_reg_op(op) as u32) | (version << 8));

        let mut ops = IrOps::new();
        ops.push(op);

        IrInst {
            cmd: load_cmd,
            ops,
            ..IrInst::default()
        }
    }
}
