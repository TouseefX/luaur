use crate::functions::writeu_8::writeu_8;

const DW_CFA_advance_loc1: u8 = 0x02;

pub unsafe fn advance_location(mut pos: *mut u8, offset: u32) -> *mut u8 {
    assert!(offset < 256);
    pos = writeu_8(pos, DW_CFA_advance_loc1);
    pos = writeu_8(pos, offset as u8);
    pos
}
