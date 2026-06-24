use crate::enums::abix_64::ABIX64;
use crate::functions::get_non_vol_xmm_storage_size::get_non_vol_xmm_storage_size;

pub const kStackAlign: u32 = 8;
pub const kStackLocalStorage: u32 = 8 * 3;
pub const kStackSpillStorage: u32 = 8 * 13;
pub const kStackExtraArgumentStorage: u32 = 2 * 8;
pub const kStackRegHomeStorage: u32 = 4 * 8;
pub const kStackOffsetToLocals: u32 = kStackExtraArgumentStorage + kStackRegHomeStorage;
pub const kStackOffsetToSpillSlots: u32 = kStackOffsetToLocals + kStackLocalStorage;

#[inline]
pub fn get_full_stack_size(abi: ABIX64, xmm_reg_count: u8) -> u32 {
    kStackOffsetToSpillSlots
        + kStackSpillStorage
        + get_non_vol_xmm_storage_size(abi, xmm_reg_count)
        + kStackAlign
}
