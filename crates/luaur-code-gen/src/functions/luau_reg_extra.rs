use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::type_aliases::t_value::TValue;

/// Returns an operand for the extra field of a TValue in a Luau register.
///
/// C++: dword[rBase + ri * sizeof(TValue) + offsetof(TValue, extra)]
#[inline]
pub fn luau_reg_extra(ri: i32) -> OperandX64 {
    let tvalue_size = core::mem::size_of::<TValue>() as i32;
    let extra_offset = core::mem::offset_of!(TValue, extra) as i32;

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        SizeX64::dword,
        RegisterX64::noreg,
        1,
        RegisterX64::r14,
        ri * tvalue_size + extra_offset,
    )
}
