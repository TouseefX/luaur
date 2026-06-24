use crate::macros::avx_w::AVX_W;
use crate::records::register_x_64::RegisterX64;

#[allow(non_snake_case)]
#[inline(always)]
pub const fn AVX_3_3(w: bool, v: RegisterX64, l: u8, p: u8) -> u8 {
    AVX_W(w) | ((!(v.index() & 0xf) & 0xf) << 3) | ((l as u8) << 2) | p
}
