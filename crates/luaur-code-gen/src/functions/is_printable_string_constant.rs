pub fn is_printable_string_constant(str_ptr: *const core::ffi::c_char, len: usize) -> bool {
    for i in 0..len {
        unsafe {
            let byte = *str_ptr.add(i) as u8;
            if byte < b' ' {
                return false;
            }
        }
    }
    true
}
