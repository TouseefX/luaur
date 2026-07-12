use crate::records::lint_format_string::LintFormatString;
use core::ffi::c_char;

impl LintFormatString {
    #[inline]
    pub fn check_string_pack(
        &self,
        data: *const c_char,
        size: usize,
        fixed: bool,
    ) -> *const c_char {
        let options = b"<>!=bBhHlLjJTiIfdnczsxX ";
        let unsized_opts = b"<>!zX ";

        let mut i = 0;
        while i < size {
            let ch = unsafe { *data.add(i) };

            if !options.contains(&(ch as u8)) {
                return c"unexpected character; must be a pack specifier or space".as_ptr();
            }

            if ch == b'c' as core::ffi::c_char
                && (i + 1 == size || !self.is_digit(unsafe { *data.add(i + 1) }))
            {
                return c"fixed-sized string format must specify the size".as_ptr();
            }

            if ch == b'X' as core::ffi::c_char
                && (i + 1 == size || unsized_opts.contains(&(unsafe { *data.add(i + 1) } as u8)))
            {
                return c"X must be followed by a size specifier".as_ptr();
            }

            if fixed && (ch == b'z' as core::ffi::c_char || ch == b's' as core::ffi::c_char) {
                return c"pack specifier must be fixed-size".as_ptr();
            }

            if (ch == b'!' as core::ffi::c_char
                || ch == b'i' as core::ffi::c_char
                || ch == b'I' as core::ffi::c_char
                || ch == b'c' as core::ffi::c_char
                || ch == b's' as core::ffi::c_char)
                && i + 1 < size
                && self.is_digit(unsafe { *data.add(i + 1) })
            {
                let isc = ch == b'c' as core::ffi::c_char;

                let mut v: u32 = 0;
                while i + 1 < size
                    && self.is_digit(unsafe { *data.add(i + 1) })
                    && v <= (i32::MAX as u32 - 9) / 10
                {
                    let digit_ch = unsafe { *data.add(i + 1) };
                    v = v * 10 + (digit_ch as u8 - b'0') as u32;
                    i += 1;
                }

                if i + 1 < size && self.is_digit(unsafe { *data.add(i + 1) }) {
                    return c"size specifier is too large".as_ptr();
                }

                if !isc && (v == 0 || v > 16) {
                    return c"integer size must be in range [1,16]".as_ptr();
                }
            }

            i += 1;
        }

        core::ptr::null()
    }
}
