use luaur_common::LUAU_ASSERT;

impl crate::records::lint_format_string::LintFormatString {
    #[inline]
    pub fn is_digit(&self, ch: core::ffi::c_char) -> bool {
        // use unsigned comparison to do range check for performance
        let _ = self;
        LUAU_ASSERT!(true);
        ((ch as u8).wrapping_sub(b'0')) < 10
    }
}
