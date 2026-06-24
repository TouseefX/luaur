use crate::records::register_x_64::RegisterX64;

#[allow(non_snake_case)]
pub const fn AVX_X(reg: RegisterX64) -> u8 {
    ((!reg.index() & 0x8) << 3) as u8
}
