use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

pub const fn operator_add_register_x_64_i32(reg: RegisterX64, disp: i32) -> OperandX64 {
    OperandX64::mem(SizeX64::none, RegisterX64::noreg, 1, reg, disp)
}
