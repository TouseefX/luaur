#[inline]
pub unsafe fn writeu_16(target: *mut u8, value: u16) -> *mut u8 {
    let le_value = value.to_le();
    core::ptr::copy_nonoverlapping(
        &le_value as *const u16 as *const u8,
        target,
        core::mem::size_of::<u16>(),
    );
    target.add(core::mem::size_of::<u16>())
}
