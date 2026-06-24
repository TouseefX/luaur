use crate::macros::rex_b::REX_B;
use crate::records::register_x_64::RegisterX64;

#[allow(non_snake_case)]
pub const fn REX_X(reg: RegisterX64) -> u8 {
    ((reg.index() & 0x8) >> 2) as u8
}
