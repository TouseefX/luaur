use crate::records::boost_like_reporter::BoostLikeReporter;

pub mod doctest {
    pub struct MessageData {
        _private: [u8; 0],
    }
}

pub fn boost_like_reporter_log_message(reporter: &BoostLikeReporter, md: &doctest::MessageData) {
    let _ = reporter;

    unsafe {
        let md_ptr = md as *const doctest::MessageData as *const u8;

        // doctest::String is at offset 0 (size 24)
        // On 64-bit, if (buf[23] & 128) == 0, it's inline (on stack), otherwise it's on heap (pointer at offset 0)
        let last_byte = *md_ptr.add(23);
        let c_str_ptr = if (last_byte & 128) == 0 {
            md_ptr as *const core::ffi::c_char
        } else {
            *(md_ptr as *const *const core::ffi::c_char)
        };

        // m_file is at offset 24 (size 8)
        let m_file = *(md_ptr.add(24) as *const *const core::ffi::c_char);

        // m_line is at offset 32 (size 4)
        let m_line = *(md_ptr.add(32) as *const i32);

        // m_severity is at offset 36 (size 4)
        let m_severity = *(md_ptr.add(36) as *const i32);

        let severity = if (m_severity & 1) != 0 {
            "WARNING"
        } else {
            "ERROR"
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

        println!("{}({}): {}: {}", file_str, m_line, severity, msg_str);
    }
}
