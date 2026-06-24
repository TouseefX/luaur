use crate::records::team_city_reporter::TeamCityReporter;
use core::ffi::c_char;

pub fn team_city_reporter_log_assert(reporter: &TeamCityReporter, ad: *const core::ffi::c_void) {
    let _ = reporter;

    unsafe {
        // ad is a doctest::AssertData const& — access fields via the C++ ABI.
        //
        // Approx layout used by doctest:
        // struct AssertData {
        //     const char* m_file;
        //     int m_line;
        //     const char* m_expr;
        //     std::string m_decomp;
        //     bool m_failed;
        // };
        //
        // We read based on the offsets used by the existing translation patterns.
        // Specifically, assume:
        // - m_failed at offset 32 (1 byte / bool in surrounding padding)
        // - m_file at offset 0
        // - m_line at offset 8
        // - m_expr at offset 16
        // - m_decomp at offset 24 (std::string: pointer/size/cap or similar)
        let m_failed = *(ad.add(32) as *const u8) != 0;
        if !m_failed {
            return;
        }

        let m_file = *(ad.add(0) as *const *const c_char);
        let m_line = *(ad.add(8) as *const i32);
        let m_expr = *(ad.add(16) as *const *const c_char);

        // Detect whether m_decomp.size() is non-zero.
        let m_decomp_size = *(ad.add(24 + 8) as *const usize);
        if m_decomp_size != 0 {
            let m_decomp_ptr = *(ad.add(24) as *const *const c_char);

            let file_str = if m_file.is_null() {
                String::new()
            } else {
                core::ffi::CStr::from_ptr(m_file)
                    .to_string_lossy()
                    .into_owned()
            };
            let expr_str = if m_expr.is_null() {
                String::new()
            } else {
                core::ffi::CStr::from_ptr(m_expr)
                    .to_string_lossy()
                    .into_owned()
            };
            let decomp_str = if m_decomp_ptr.is_null() {
                String::new()
            } else {
                core::ffi::CStr::from_ptr(m_decomp_ptr)
                    .to_string_lossy()
                    .into_owned()
            };

            eprintln!(
                "{}({}): ERROR: {} ({})",
                file_str, m_line, expr_str, decomp_str
            );
        } else {
            let file_str = if m_file.is_null() {
                String::new()
            } else {
                core::ffi::CStr::from_ptr(m_file)
                    .to_string_lossy()
                    .into_owned()
            };
            let expr_str = if m_expr.is_null() {
                String::new()
            } else {
                core::ffi::CStr::from_ptr(m_expr)
                    .to_string_lossy()
                    .into_owned()
            };

            eprintln!("{}({}): ERROR: {}", file_str, m_line, expr_str);
        }
    }
}
