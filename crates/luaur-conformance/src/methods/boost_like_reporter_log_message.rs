use crate::records::boost_like_reporter::BoostLikeReporter;

impl BoostLikeReporter {
    #[allow(non_snake_case)]
    pub fn log_message(&mut self, md: &doctest::MessageData) {
        let severity = if (md.m_severity & doctest::assertType::is_warn) != 0 {
            "WARNING"
        } else {
            "ERROR"
        };

        unsafe {
            let file = core::ffi::CStr::from_ptr(md.m_file).to_string_lossy();
            let message = core::ffi::CStr::from_ptr(md.m_string.c_str()).to_string_lossy();

            println!("{}({}): {}: {}", file, md.m_line, severity, message);
        }
    }
}

/// Doctest minimal bindings for the reporter implementation.
mod doctest {
    use core::ffi::c_char;

    #[repr(C)]
    pub struct MessageData {
        pub m_severity: i32,
        pub m_file: *const c_char,
        pub m_line: i32,
        pub m_string: StringProxy,
    }

    #[repr(C)]
    pub struct StringProxy {
        pub data: *const c_char,
    }

    impl StringProxy {
        pub fn c_str(&self) -> *const c_char {
            self.data
        }
    }

    pub mod assertType {
        pub const is_warn: i32 = 1;
    }
}
