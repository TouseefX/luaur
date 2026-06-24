use crate::records::lint_format_string::LintFormatString;
use core::ffi::c_char;

pub fn fuzz_format_string(data: *const c_char, size: usize) {
    let lfs = LintFormatString {
        context: core::ptr::null_mut(),
    };
    lfs.fuzz(data, size);
}
