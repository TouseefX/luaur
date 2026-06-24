use crate::enums::ir_cmd::IrCmd;
use crate::records::const_prop_state::ConstPropState;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;

impl ConstPropState {
    pub fn versioned_vm_reg_load_ir_cmd_ir_op_ir_op(
        &mut self,
        load_cmd: IrCmd,
        op_a: IrOp,
        op_b: IrOp,
    ) -> IrInst {
        let mut inst = self.versioned_vm_reg_load_ir_cmd_ir_op(load_cmd, op_a);
        inst.ops.push(op_b);
        inst
    }
}
