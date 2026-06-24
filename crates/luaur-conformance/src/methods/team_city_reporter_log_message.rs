use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::{c_char, CStr};

impl TeamCityReporter {
    #[allow(non_snake_case)]
    pub fn log_message_impl(&mut self, md: &doctest::MessageData) {
        let severity = if (md.m_severity & doctest::assertType::is_warn) != 0 {
            "WARNING"
        } else {
            "ERROR"
        };

        let is_error = (md.m_severity & doctest::assertType::is_require) != 0
            || (md.m_severity & doctest::assertType::is_check) != 0;

        unsafe {
            let file = CStr::from_ptr(md.m_file).to_string_lossy();
            let message = CStr::from_ptr(md.m_string.c_str()).to_string_lossy();

            if is_error {
                eprintln!("{}({}): {}: {}", file, md.m_line, severity, message);
            } else {
                println!("{}({}): {}: {}", file, md.m_line, severity, message);
            }
        }
    }
}

pub mod doctest {
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
        pub const is_require: i32 = 2;
        pub const is_check: i32 = 4;
    }
}
