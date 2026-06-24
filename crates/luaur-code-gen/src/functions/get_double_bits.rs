pub fn get_double_bits(value: f64) -> u64 {
    let mut result: u64 = 0;
    unsafe {
        core::ptr::copy_nonoverlapping(
            &value as *const f64 as *const u8,
            &mut result as *mut u64 as *mut u8,
            core::mem::size_of::<f64>(),
        );
    }
    result
}
