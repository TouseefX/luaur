use alloc::string::String;
use core::ffi::c_char;

use crate::records::config_table_key::ConfigTableKey;

impl ConfigTableKey {
    pub fn config_table_key_c_char(s: *const c_char) -> Self {
        if s.is_null() {
            return ConfigTableKey::config_table_key();
        }
        let s_str = unsafe { core::ffi::CStr::from_ptr(s).to_string_lossy() };
        ConfigTableKey::config_table_key_string(s_str.into_owned())
    }
}
