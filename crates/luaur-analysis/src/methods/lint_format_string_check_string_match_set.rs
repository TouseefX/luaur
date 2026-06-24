use crate::records::lint_format_string::LintFormatString;

impl LintFormatString {
    #[inline]
    pub fn check_string_match_set(
        &self,
        data: *const core::ffi::c_char,
        size: usize,
        magic: &[u8],
        classes: &[u8],
    ) -> *const core::ffi::c_char {
        let mut i = 0;
        unsafe {
            while i < size {
                let ch = *data.add(i);
                if ch == b'%' as i8 {
                    i += 1;

                    if i == size {
                        return c"unfinished character class".as_ptr();
                    }

                    let next_ch = *data.add(i);
                    if self.is_digit(next_ch) {
                        return c"sets can not contain capture references".as_ptr();
                    } else if self.is_alpha(next_ch) {
                        // lower case lookup - upper case for every character class is defined as its inverse
                        if !classes.contains(&((next_ch as u8) | b' ')) {
                            return c"invalid character class, must refer to a defined class or its inverse".as_ptr();
                        }
                    } else {
                        // technically % can escape any non-alphanumeric character but this is error-prone
                        if !magic.contains(&(next_ch as u8)) {
                            return c"expected a magic character after %".as_ptr();
                        }
                    }

                    if i + 1 < size && *data.add(i + 1) == b'-' as i8 {
                        return c"character range can't include character sets".as_ptr();
                    }
                } else if ch == b'-' as i8 {
                    if i + 1 < size && *data.add(i + 1) == b'%' as i8 {
                        return c"character range can't include character sets".as_ptr();
                    }
                }

                i += 1;
            }
        }

        core::ptr::null()
    }
}
