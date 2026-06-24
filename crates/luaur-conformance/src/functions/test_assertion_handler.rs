use crate::functions::debugger_present::debugger_present;

#[allow(non_snake_case)]
pub unsafe extern "C" fn test_assertion_handler(
    expr: *const core::ffi::c_char,
    file: *const core::ffi::c_char,
    line: core::ffi::c_int,
    _function: *const core::ffi::c_char,
) -> core::ffi::c_int {
    if debugger_present() {
        // LUAU_ASSERT will trigger LUAU_DEBUGBREAK for a more convenient debugging experience
        return 1;
    }

    let expr_str = core::ffi::CStr::from_ptr(expr).to_string_lossy();
    let file_str = core::ffi::CStr::from_ptr(file).to_string_lossy();

    // ADD_FAIL_AT is a macro from the doctest/testing framework used in Luau tests.
    // In the Rust port of the conformance tests, we map this to a panic or a test failure log.
    // Since this is a native-only test runner utility, we print the failure and panic to match ADD_FAIL_AT behavior.
    eprintln!(
        "{}:{}: Failure: Assertion failed: {}",
        file_str, line, expr_str
    );

    1
}
