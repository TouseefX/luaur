use core::ffi::c_char;

impl crate::records::lint_format_string::LintFormatString {
    #[inline]
    pub fn check_string_replace(
        &self,
        data: *const c_char,
        size: usize,
        captures: i32,
    ) -> *const c_char {
        let mut i = 0;
        while i < size {
            if unsafe { *data.add(i) } == b'%' as i8 {
                i += 1;

                if i == size {
                    return c"unfinished replacement".as_ptr();
                }

                let next_ch = unsafe { *data.add(i) };
                if next_ch != b'%' as i8 && !self.is_digit(next_ch) {
                    return c"unexpected replacement character; must be a digit or %".as_ptr();
                }

                if self.is_digit(next_ch)
                    && captures >= 0
                    && (next_ch as u8 - b'0') as i32 > captures
                {
                    return c"invalid capture index, must refer to pattern capture".as_ptr();
                }
            }

            i += 1;
        }

        core::ptr::null()
    }
}
