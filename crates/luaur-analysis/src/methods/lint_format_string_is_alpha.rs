impl crate::records::lint_format_string::LintFormatString {
    #[inline]
    pub fn is_alpha(&self, ch: core::ffi::c_char) -> bool {
        ((ch as u8 | b' ').wrapping_sub(b'a')) < 26
    }
}
