use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

#[inline]
pub fn luau_reg_address(ri: i32) -> OperandX64 {
    let tvalue_size = core::mem::size_of::<luaur_vm::type_aliases::t_value::TValue>() as i32;

    let base = OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::none,
        RegisterX64::noreg,
        1,
        RegisterX64::r14,
        ri * tvalue_size,
    );

    base
}
