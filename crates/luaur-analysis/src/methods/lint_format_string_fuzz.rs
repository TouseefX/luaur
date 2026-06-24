impl crate::records::lint_format_string::LintFormatString {
    pub fn fuzz(&self, data: *const core::ffi::c_char, size: usize) {
        let mut pass = self.clone();
        pass.context = core::ptr::null_mut();

        pass.check_string_format(data, size);
        pass.check_string_pack(data, size, false);
        pass.check_string_match(data, size, core::ptr::null_mut());
        pass.check_string_replace(data, size, -1);
        pass.check_date_format(data, size);
    }
}
