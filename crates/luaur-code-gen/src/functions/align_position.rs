use crate::functions::writeu_8::writeu_8;

const K_DWARF_ALIGN: usize = core::mem::size_of::<usize>();
const DW_CFA_NOP: u8 = 0;

pub unsafe fn align_position(start: *mut u8, mut pos: *mut u8) -> *mut u8 {
    let size = (pos as usize).wrapping_sub(start as usize);
    let pad = ((size + K_DWARF_ALIGN - 1) & !(K_DWARF_ALIGN - 1)) - size;

    for _ in 0..pad {
        pos = writeu_8(pos, DW_CFA_NOP);
    }

    pos
}
