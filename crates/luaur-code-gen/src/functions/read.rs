pub unsafe fn read<T>(data: *const u8, offset: &mut usize) -> T {
    let mut result = core::mem::MaybeUninit::<T>::uninit();
    core::ptr::copy_nonoverlapping(
        data.add(*offset),
        result.as_mut_ptr() as *mut u8,
        core::mem::size_of::<T>(),
    );
    *offset += core::mem::size_of::<T>();
    result.assume_init()
}
