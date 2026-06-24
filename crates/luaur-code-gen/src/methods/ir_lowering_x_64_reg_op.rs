use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::register_x_64::RegisterX64;

impl IrLoweringX64 {
    pub fn reg_op(&mut self, op: IrOp) -> RegisterX64 {
        let function = self.function as *mut IrFunction;
        let inst = unsafe { (*function).inst_op(op) };

        if inst.spilled || inst.needs_reload {
            self.regs.restore(inst, false);
        }

        CODEGEN_ASSERT!(inst.reg_x64 != RegisterX64::noreg);
        inst.reg_x64
    }
}
