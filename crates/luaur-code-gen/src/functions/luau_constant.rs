use crate::records::operand_x_64::OperandX64;

#[inline]
pub fn luau_constant(ki: i32) -> OperandX64 {
    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        crate::enums::size_x_64::SizeX64::xmmword,
        crate::records::register_x_64::RegisterX64::noreg,
        0,
        crate::records::register_x_64::RegisterX64::rbp,
        ki,
    )
}
