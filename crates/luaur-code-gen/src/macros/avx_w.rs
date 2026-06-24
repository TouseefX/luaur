#[allow(non_snake_case)]
#[inline(always)]
pub const fn AVX_W(value: bool) -> u8 {
    if value {
        0x80
    } else {
        0x0
    }
}
