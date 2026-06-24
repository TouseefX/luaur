use crate::enums::ir_cmd::IrCmd;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_function::IrFunction;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use crate::records::scoped_reg_x_64::ScopedRegX64;

impl IrLoweringX64 {
    pub fn vec_op(&mut self, op: IrOp, tmp: &mut ScopedRegX64) -> RegisterX64 {
        let function = self.function as *mut IrFunction;
        let source = unsafe { (*function).inst_op(op) };

        CODEGEN_ASSERT!(source.cmd != IrCmd::SUBSTITUTE);

        if source.cmd != IrCmd::LOAD_TVALUE
            && source.cmd != IrCmd::GET_UPVALUE
            && source.cmd != IrCmd::TAG_VECTOR
        {
            return self.reg_op(op);
        }

        tmp.alloc(SizeX64::xmmword);
        let dst = OperandX64::reg(tmp.reg);
        let src1 = OperandX64::reg(self.reg_op(op));
        let src2 = self.vector_and_mask_op();
        unsafe { (*self.build).vandps(dst, src1, src2) };
        tmp.reg
    }
}
