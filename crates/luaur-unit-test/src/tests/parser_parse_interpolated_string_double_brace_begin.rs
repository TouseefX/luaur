#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_double_brace_begin() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    let source = alloc::string::String::from("\n            _ = `{{oops}}`\n        ");
    let parse_options = luaur_ast::records::parse_options::ParseOptions::default();
    // C++ catches the thrown ParseErrors. `fixture.parse` PANICS on errors (it
    // emulates the throw), so don't call it — use try_parse and assert the error,
    // which `errors.first().unwrap()` below already enforces.
    let parse_result = fixture.try_parse(&source, &parse_options);
    let errors = parse_result.errors;
    let first_error = errors.first().unwrap();
    let message = first_error.get_message();

    let expected_message =
        "Double braces are not permitted within interpolated strings; did you mean '\\{'?";
    if message.as_str() != expected_message {
        panic!(
            "Expected error message '{}', got '{}'",
            expected_message,
            unsafe {
                core::ffi::CStr::from_ptr(message.as_ptr() as *const core::ffi::c_char)
                    .to_string_lossy()
            }
        );
    }
}
