use crate::records::ir_const::IrConst;
use crate::records::ir_function::IrFunction;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_const_op(&self, op: IrOp) -> IrConst {
        unsafe { (*self.function).const_op(op) }
    }
}
