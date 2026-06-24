use crate::records::lint_format_string::LintFormatString;
use core::ffi::c_char;

impl LintFormatString {
    pub fn check_date_format(&self, data: *const c_char, size: usize) -> *const c_char {
        let options = b"aAbBcdHIjmMpSUwWxXyYzZ";

        let mut i = 0;
        while i < size {
            let ch = unsafe { *data.add(i) };
            if ch == b'%' as i8 {
                i += 1;

                if i == size {
                    return c"unfinished replacement".as_ptr();
                }

                let next_ch = unsafe { *data.add(i) };
                if next_ch != b'%' as i8 && !options.contains(&(next_ch as u8)) {
                    return c"unexpected replacement character; must be a date format specifier or %".as_ptr();
                }
            }

            if unsafe { *data.add(i) } == 0 {
                return c"date format can not contain null characters".as_ptr();
            }
            i += 1;
        }

        core::ptr::null()
    }
}
