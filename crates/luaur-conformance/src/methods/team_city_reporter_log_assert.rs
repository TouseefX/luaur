use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::{c_char, CStr};

impl TeamCityReporter {
    pub fn log_assert_impl(&mut self, ad: &doctest::AssertData) {
        if !ad.m_failed {
            return;
        }

        unsafe {
            let file = CStr::from_ptr(ad.m_file).to_string_lossy();
            let expr = CStr::from_ptr(ad.m_expr).to_string_lossy();

            if ad.m_decomp.size() > 0 {
                let decomp = CStr::from_ptr(ad.m_decomp.c_str()).to_string_lossy();
                eprintln!("{}({}): ERROR: {} ({})", file, ad.m_line, expr, decomp);
            } else {
                eprintln!("{}({}): ERROR: {}", file, ad.m_line, expr);
            }
        }
    }
}

/// Doctest minimal bindings for the reporter implementation.
pub mod doctest {
    use core::ffi::c_char;

    #[repr(C)]
    pub struct AssertData {
        pub m_failed: bool,
        pub m_file: *const c_char,
        pub m_line: i32,
        pub m_expr: *const c_char,
        pub m_decomp: StringProxy,
    }

    #[repr(C)]
    pub struct StringProxy {
        pub data: *const c_char,
        pub size: usize,
    }

    impl StringProxy {
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn c_str(&self) -> *const c_char {
            self.data
        }
    }
}
