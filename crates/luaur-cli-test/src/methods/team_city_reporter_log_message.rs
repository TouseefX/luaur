use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::{c_char, c_void};

pub mod doctest {
    pub struct MessageData {
        _private: [u8; 0],
    }
}

pub fn team_city_reporter_log_message(_reporter: &TeamCityReporter, md: &doctest::MessageData) {
    unsafe {
        let md_ptr = md as *const doctest::MessageData as *const u8;

        // doctest::MessageData layout (v2.4.11):
        // struct MessageData {
        //     String m_string; // 24 bytes
        //     const char* m_file; // 8 bytes
        //     int m_line; // 4 bytes
        //     int m_severity; // 4 bytes
        // };
        // Offsets: m_string: 0, m_file: 24, m_line: 32, m_severity: 36

        let m_string_ptr = md_ptr.add(0);
        let m_file = *(md_ptr.add(24) as *const *const c_char);
        let m_line = *(md_ptr.add(32) as *const i32);
        let m_severity = *(md_ptr.add(36) as *const i32);

        // doctest::assertType::is_warn = 1
        // doctest::assertType::is_require = 2
        // doctest::assertType::is_check = 4
        let is_warn = (m_severity & 1) != 0;
        let is_error = (m_severity & (2 | 4)) != 0;

        let severity = if is_warn { "WARNING" } else { "ERROR" };

        // Access doctest::String (inline or heap)
        let last_byte = *m_string_ptr.add(23);
        let c_str_ptr = if (last_byte & 128) == 0 {
            m_string_ptr as *const c_char
        } else {
            *(m_string_ptr as *const *const c_char)
        };

        let file_str = if m_file.is_null() {
            std::borrow::Cow::Borrowed("")
        } else {
            core::ffi::CStr::from_ptr(m_file).to_string_lossy()
        };

        let msg_str = if c_str_ptr.is_null() {
            std::borrow::Cow::Borrowed("")
        } else {
            core::ffi::CStr::from_ptr(c_str_ptr).to_string_lossy()
        };

        if is_error {
            eprintln!("{}({}): {}: {}", file_str, m_line, severity, msg_str);
        } else {
            println!("{}({}): {}: {}", file_str, m_line, severity, msg_str);
        }
    }
}
