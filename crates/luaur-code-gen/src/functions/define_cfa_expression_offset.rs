use crate::functions::writeu_8::writeu_8;
use crate::functions::writeuleb_128::writeuleb_128;

pub unsafe fn define_cfa_expression_offset(mut pos: *mut u8, stack_offset: u32) -> *mut u8 {
    const DW_CFA_def_cfa_offset: u8 = 0x0e;
    pos = writeu_8(pos, DW_CFA_def_cfa_offset);
    pos = writeuleb_128(pos, stack_offset as u64);
    pos
}
