use crate::records::operand_x_64::{addr, OperandX64};
use crate::records::register_x_64::RegisterX64;
use luaur_vm::type_aliases::t_value::TValue;

pub fn luau_constant_address(ki: i32) -> OperandX64 {
    let tvalue_size = core::mem::size_of::<TValue>() as i32;

    addr.operand_x_64_operator_index(RegisterX64::r12 + ki * tvalue_size)
}
