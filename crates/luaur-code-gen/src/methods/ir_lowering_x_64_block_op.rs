use crate::records::ir_block::IrBlock;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;

impl IrLoweringX64 {
    pub fn block_op(&self, op: IrOp) -> &mut IrBlock {
        unsafe {
            let self_mut = self as *const Self as *mut Self;
            (*(*self_mut).function).block_op(op)
        }
    }
}
