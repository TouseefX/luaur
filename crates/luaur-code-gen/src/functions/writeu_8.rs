#[inline]
pub unsafe fn writeu_8(target: *mut u8, value: u8) -> *mut u8 {
    *target = value;
    target.add(core::mem::size_of::<u8>())
}
