use luaur_common::macros::luau_big_endian::LUAU_BIG_ENDIAN;

#[inline]
pub unsafe fn writef_32(target: *mut u8, value: f32) -> *mut u8 {
    if LUAU_BIG_ENDIAN {
        let mut data: u32 = 0;
        core::ptr::copy_nonoverlapping(
            &value as *const f32 as *const u8,
            &mut data as *mut u32 as *mut u8,
            core::mem::size_of::<f32>(),
        );
        crate::functions::writeu_32::writeu_32(target, data)
    } else {
        core::ptr::copy_nonoverlapping(
            &value as *const f32 as *const u8,
            target,
            core::mem::size_of::<f32>(),
        );
        target.add(core::mem::size_of::<f32>())
    }
}
