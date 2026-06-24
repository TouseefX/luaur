#[inline]
pub unsafe fn writeu_32(target: *mut u8, mut value: u32) -> *mut u8 {
    if luaur_common::macros::luau_big_endian::LUAU_BIG_ENDIAN {
        value = value.to_le();
    }

    core::ptr::copy_nonoverlapping(&value as *const u32 as *const u8, target, 4);
    target.add(4)
}
