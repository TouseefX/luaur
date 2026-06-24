use crate::records::lint_format_string::LintFormatString;

impl crate::records::lint_format_string::LintFormatString {
    #[inline]
    pub fn check_string_match(
        &self,
        data: *const core::ffi::c_char,
        size: usize,
        out_captures: *mut i32,
    ) -> *const core::ffi::c_char {
        let magic = b"^$()%.[]*+-?)";
        let classes = b"acdglpsuwxz";

        let mut open_captures: alloc::vec::Vec<i32> = alloc::vec::Vec::new();
        let mut total_captures: i32 = 0;

        let mut i: usize = 0;
        unsafe {
            while i < size {
                if *data.add(i) == b'%' as i8 {
                    i += 1;

                    if i == size {
                        return c"unfinished character class".as_ptr();
                    }

                    let ch = *data.add(i);
                    if self.is_digit(ch) {
                        if ch == b'0' as i8 {
                            return c"invalid capture reference, must be 1-9".as_ptr();
                        }

                        let capture_index = (ch as u8 - b'0') as i32;

                        if capture_index > total_captures {
                            return c"invalid capture reference, must refer to a valid capture"
                                .as_ptr();
                        }

                        for &open in &open_captures {
                            if open == capture_index {
                                return c"invalid capture reference, must refer to a closed capture".as_ptr();
                            }
                        }
                    } else if self.is_alpha(ch) {
                        if ch == b'b' as i8 {
                            if i + 2 >= size {
                                return c"missing brace characters for balanced match".as_ptr();
                            }
                            i += 2;
                        } else if ch == b'f' as i8 {
                            if i + 1 >= size || *data.add(i + 1) != b'[' as i8 {
                                return c"missing set after a frontier pattern".as_ptr();
                            }
                            // we can parse the set with the regular logic
                        } else {
                            // lower case lookup - upper case for every character class is defined as its inverse
                            if !classes.contains(&((ch as u8) | b' ')) {
                                return c"invalid character class, must refer to a defined class or its inverse".as_ptr();
                            }
                        }
                    } else {
                        // technically % can escape any non-alphanumeric character but this is error-prone
                        if !magic.contains(&(ch as u8)) {
                            return c"expected a magic character after %".as_ptr();
                        }
                    }
                } else if *data.add(i) == b'[' as i8 {
                    let mut j = i + 1;

                    // empty patterns don't exist as per grammar rules, so we skip leading ^ and ]
                    if j < size && *data.add(j) == b'^' as i8 {
                        j += 1;
                    }
                    if j < size && *data.add(j) == b']' as i8 {
                        j += 1;
                    }

                    // scan for the end of the pattern
                    while j < size && *data.add(j) != b']' as i8 {
                        // % escapes the next character
                        if j + 1 < size && *data.add(j) == b'%' as i8 {
                            j += 1;
                        }
                        j += 1;
                    }

                    if j == size {
                        return c"expected ] at the end of the string to close a set".as_ptr();
                    }

                    let error =
                        self.check_string_match_set(data.add(i + 1), j - i - 1, magic, classes);
                    if !error.is_null() {
                        return error;
                    }

                    debug_assert!(*data.add(j) == b']' as i8);
                    i = j;
                } else if *data.add(i) == b'(' as i8 {
                    total_captures += 1;
                    open_captures.push(total_captures);
                } else if *data.add(i) == b')' as i8 {
                    if open_captures.is_empty() {
                        return c"unexpected ) without a matching (".as_ptr();
                    }
                    open_captures.pop();
                }

                i += 1;
            }
        }

        if !open_captures.is_empty() {
            return c"expected ) at the end of the string to close a capture".as_ptr();
        }

        if !out_captures.is_null() {
            unsafe {
                *out_captures = total_captures;
            }
        }

        core::ptr::null()
    }
}
