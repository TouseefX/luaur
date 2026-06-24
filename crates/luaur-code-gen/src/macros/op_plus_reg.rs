#[allow(non_snake_case)]
#[inline]
pub const fn OP_PLUS_REG(op: u8, reg: u8) -> u8 {
    op.wrapping_add(reg & 0x7)
}
