use crate::records::operand_x_64::OperandX64;
use luaur_vm::type_aliases::t_value::TValue;

#[inline]
pub fn luau_reg_tag(ri: i32) -> OperandX64 {
    let tvalue_size = core::mem::size_of::<TValue>() as i32;
    let tt_offset = core::mem::offset_of!(TValue, tt) as i32;

    OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        crate::enums::size_x_64::SizeX64::dword,
        crate::records::register_x_64::RegisterX64::noreg,
        1,
        crate::records::register_x_64::RegisterX64::r14,
        ri * tvalue_size + tt_offset,
    )
}
