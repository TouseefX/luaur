use crate::records::stringifier_state::StringifierState;
use core::ffi::c_char;

impl StringifierState {
    pub fn emit_c_char(&mut self, s: *const c_char) {
        if self.opts.is_null() {
            return;
        }

        let max_type_length = unsafe { (*self.opts).max_type_length };
        if max_type_length > 0 {
            let result_name = unsafe { &(*self.result).name };
            if result_name.len() > max_type_length as usize {
                return;
            }
        }

        if s.is_null() {
            return;
        }

        let slice = unsafe {
            let mut len = 0;
            while *s.add(len) != 0 {
                len += 1;
            }
            core::slice::from_raw_parts(s as *const u8, len)
        };

        let s_str = core::str::from_utf8(slice).unwrap_or("");
        self.emit_string(s_str);
    }
}
