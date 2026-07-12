use crate::records::lint_format_string::LintFormatString;
use core::ffi::c_char;

impl LintFormatString {
    pub fn check_string_format(&self, data: *const c_char, size: usize) -> *const c_char {
        let flags = b"-+ #0";
        let options = b"cdiouxXeEfgGqs*";

        let mut i = 0;
        while i < size {
            let ch = unsafe { *data.add(i) };
            if ch == b'%' as core::ffi::c_char {
                i += 1;

                if i < size && unsafe { *data.add(i) } == b'%' as core::ffi::c_char {
                    i += 1;
                    continue;
                }

                while i < size && flags.contains(&(unsafe { *data.add(i) } as u8)) {
                    i += 1;
                }

                if i < size && self.is_digit(unsafe { *data.add(i) }) {
                    i += 1;
                }
                if i < size && self.is_digit(unsafe { *data.add(i) }) {
                    i += 1;
                }

                if i < size && unsafe { *data.add(i) } == b'.' as core::ffi::c_char {
                    i += 1;

                    if i < size && self.is_digit(unsafe { *data.add(i) }) {
                        i += 1;
                    }
                    if i < size && self.is_digit(unsafe { *data.add(i) }) {
                        i += 1;
                    }
                }

                if i == size {
                    return c"unfinished format specifier".as_ptr();
                }

                if !options.contains(&(unsafe { *data.add(i) } as u8)) {
                    return c"invalid format specifier: must be a string format specifier or %"
                        .as_ptr();
                }
            }
            i += 1;
        }

        core::ptr::null()
    }
}
