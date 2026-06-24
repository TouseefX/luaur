use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;

#[allow(non_snake_case)]
pub fn OPT_OP_B(inst: IrInst) -> IrOp {
    if 1 < inst.ops.size() && inst.ops[1].kind() != IrOpKind::None {
        inst.ops[1]
    } else {
        IrOp { kind_and_index: 0 }
    }
}
