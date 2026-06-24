use crate::macros::avx_b::AVX_B;
use crate::macros::avx_r::AVX_R;
use crate::macros::avx_x::AVX_X;
use crate::records::register_x_64::RegisterX64;

#[allow(non_snake_case)]
#[inline(always)]
pub const fn AVX_3_2(r: RegisterX64, x: RegisterX64, b: RegisterX64, m: u8) -> u8 {
    AVX_R(r) | AVX_X(x) | AVX_B(b) | m
}
