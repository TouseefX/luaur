use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;

pub fn operator_add_operand_x_64_i32(mut op: OperandX64, disp: i32) -> OperandX64 {
    debug_assert!(op.cat == CategoryX64::mem);
    debug_assert!(op.memSize == SizeX64::none);

    op.imm += disp;
    op
}
