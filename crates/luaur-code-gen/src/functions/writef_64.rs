#[inline]
pub unsafe fn writef_64(target: *mut u8, value: f64) -> *mut u8 {
    if luaur_common::macros::luau_big_endian::LUAU_BIG_ENDIAN {
        let mut data: u64 = 0;
        core::ptr::copy_nonoverlapping(
            &value as *const f64 as *const u8,
            &mut data as *mut u64 as *mut u8,
            core::mem::size_of::<f64>(),
        );
        crate::functions::writeu_64::writeu_64(target, data)
    } else {
        core::ptr::copy_nonoverlapping(
            &value as *const f64 as *const u8,
            target,
            core::mem::size_of::<f64>(),
        );
        target.add(core::mem::size_of::<f64>())
    }
}
