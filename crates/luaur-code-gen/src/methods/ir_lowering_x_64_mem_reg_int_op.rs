use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::luau_reg_value_int::luau_reg_value_int;
use crate::functions::vm_reg_op::vm_reg_op;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::operand_x_64::OperandX64;

impl IrLoweringX64 {
    pub fn mem_reg_int_op(&mut self, op: IrOp) -> OperandX64 {
        match op.kind() {
            IrOpKind::Inst => OperandX64::operand_x_64_register_x_64(self.reg_op(op)),
            IrOpKind::Constant => OperandX64::operand_x_64_i32(self.int_op(op)),
            IrOpKind::VmReg => luau_reg_value_int(vm_reg_op(op)),
            _ => {
                CODEGEN_ASSERT!(false);
                OperandX64::operand_x_64_register_x_64(
                    crate::records::register_x_64::RegisterX64::noreg,
                )
            }
        }
    }
}
