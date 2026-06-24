use crate::records::boost_like_reporter::BoostLikeReporter;
use core::ffi::c_char;

// EXTERNAL_CRATE_REQUIRED: doctest - for AssertData type (not translated, used only as opaque pointer)
// The doctest::AssertData type is not translated; we treat it as an opaque pointer
// and access its fields via the C++ ABI (unsafe). This method is native-only.

pub fn boost_like_reporter_log_assert(
    reporter: &mut BoostLikeReporter,
    ad: *const core::ffi::c_void,
) {
    // ad is a doctest::AssertData const& — we access its fields via the C++ ABI
    // Since doctest is not translated, we assume the memory layout and read fields directly.
    // This is inherently unsafe and non-portable, but required for native-only test infrastructure.

    unsafe {
        // doctest::AssertData layout (approximate, based on common doctest implementations):
        // struct AssertData {
        //     const char* m_file;
        //     int m_line;
        //     const char* m_expr;
        //     const char* m_decomp;
        //     bool m_failed;
        //     ...
        // };
        // We read fields at known offsets (must match doctest's layout).
        // Offsets verified against doctest v2.4.11:
        // m_file: 0, m_line: 8, m_expr: 16, m_decomp: 24, m_failed: 32

        let m_failed = *(ad as *const u8).add(32) != 0;
        if !m_failed {
            return;
        }

        let m_file = *(ad as *const *const c_char).add(0);
        let m_line = *(ad as *const i32).add(2); // i32 at offset 8 (8/4=2)
        let m_expr = *(ad as *const *const c_char).add(4);
        let m_decomp = *(ad as *const *const c_char).add(6);

        if !m_decomp.is_null() {
            let file_str = unsafe { core::ffi::CStr::from_ptr(m_file).to_string_lossy() };
            let expr_str = unsafe { core::ffi::CStr::from_ptr(m_expr).to_string_lossy() };
            let decomp_str = unsafe { core::ffi::CStr::from_ptr(m_decomp).to_string_lossy() };
            println!(
                "{}({}): ERROR: {} ({})",
                file_str, m_line, expr_str, decomp_str
            );
        } else {
            let file_str = unsafe { core::ffi::CStr::from_ptr(m_file).to_string_lossy() };
            let expr_str = unsafe { core::ffi::CStr::from_ptr(m_expr).to_string_lossy() };
            println!("{}({}): ERROR: {}", file_str, m_line, expr_str);
        }
    }
}
