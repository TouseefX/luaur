use crate::functions::writeu_8::writeu_8 as writeu8;
use crate::functions::writeuleb_128::writeuleb_128 as writeuleb128;

const DW_CFA_def_cfa: u8 = 0x0c;

pub unsafe fn define_cfa_expression(mut pos: *mut u8, dw_reg: i32, stack_offset: u32) -> *mut u8 {
    pos = writeu8(pos, DW_CFA_def_cfa);
    pos = writeuleb128(pos, dw_reg as u64);
    pos = writeuleb128(pos, stack_offset as u64);
    pos
}
