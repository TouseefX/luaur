use crate::enums::abix_64::ABIX64;

pub const kSystemVUsableXmmRegs: u8 = 16;
pub const kWindowsUsableXmmRegs: u8 = 10;

#[inline]
pub fn get_xmm_register_count(abi: ABIX64) -> u8 {
    if abi == ABIX64::SystemV {
        kSystemVUsableXmmRegs
    } else {
        kWindowsUsableXmmRegs
    }
}
