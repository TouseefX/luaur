#[allow(non_snake_case)]
#[inline]
pub const fn OP_PLUS_CC(op: u8, cc: u8) -> u8 {
    op.wrapping_add(cc)
}
