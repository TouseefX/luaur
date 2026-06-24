use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

/// Returns an operand for the 64-bit integer part of a TValue in a Luau register.
///
/// C++: qword[rBase + ri * sizeof(TValue) + offsetof(TValue, value.l)]
///
#[inline]
pub fn luau_reg_value_int_64(ri: i32) -> OperandX64 {
    // TValue size is 16 bytes in Luau VM (Value value + int extra[2] + int tt)
    let tvalue_size = core::mem::size_of::<luaur_vm::type_aliases::t_value::TValue>() as i32;
    // offsetof(TValue, value.l) is 8 (Value.l is the 64-bit integer member)
    let value_l_offset = 8;

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        RegisterX64::r14,
        ri * tvalue_size + value_l_offset,
    )
}
