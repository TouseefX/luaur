use crate::functions::writeu_8::writeu_8 as writeu8;
use crate::functions::writeuleb_128::writeuleb_128 as writeuleb128;

const DW_CFA_offset: u8 = 0x80;
const DW_CFA_offset_extended: u8 = 0x05;
const kDataAlignFactor: u32 = 8;

pub unsafe fn define_saved_register_location(
    mut pos: *mut u8,
    dw_reg: i32,
    stack_offset: u32,
) -> *mut u8 {
    assert!(
        stack_offset % kDataAlignFactor == 0,
        "stack offsets have to be measured in kDataAlignFactor units"
    );

    if dw_reg <= 0x3f {
        pos = writeu8(pos, DW_CFA_offset + dw_reg as u8);
    } else {
        pos = writeu8(pos, DW_CFA_offset_extended);
        pos = writeuleb128(pos, dw_reg as u64);
    }

    pos = writeuleb128(pos, (stack_offset / kDataAlignFactor) as u64);
    pos
}
