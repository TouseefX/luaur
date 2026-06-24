#[cfg(test)]
#[test]
fn parser_error_on_confusable() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // C++ source is `local pi = 3․13` (U+2024 ONE DOT LEADER, a `.` confusable).
    // The ported literal was double-encoded UTF-8 mojibake; use a \u escape.
    fixture.match_parse_error(
        &alloc::string::String::from("\n        local pi = 3\u{2024}13\n    "),
        &alloc::string::String::from("Expected identifier when parsing expression, got Unicode character U+2024 (did you mean '.'?)"),
        None,
    );
}
