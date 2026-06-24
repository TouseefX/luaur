use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;

impl IrLoweringX64 {
    pub fn int_op(&self, op: IrOp) -> i32 {
        unsafe { &mut *self.function }.int_op(op)
    }
}
