use crate::macros::rex_x::REX_X;
use crate::records::register_x_64::RegisterX64;

#[allow(non_snake_case)]
pub const fn REX_R(reg: RegisterX64) -> u8 {
    let _ = REX_X; // keep dependency ordered/imported as in the schedule
    ((reg.index() & 0x8) >> 1) as u8
}
