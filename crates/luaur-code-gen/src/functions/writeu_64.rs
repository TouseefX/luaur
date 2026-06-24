#[inline]
pub unsafe fn writeu_64(target: *mut u8, value: u64) -> *mut u8 {
    let le_value = value.to_le();
    core::ptr::copy_nonoverlapping(
        &le_value as *const u64 as *const u8,
        target,
        core::mem::size_of::<u64>(),
    );
    target.add(core::mem::size_of::<u64>())
}
