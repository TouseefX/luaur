#[cfg(test)]
#[test]
fn parser_error_on_unicode() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    // C++ source is `local ☃ = 10` (U+2603 SNOWMAN). The ported literal was
    // double-encoded UTF-8 mojibake; use an explicit \u escape (encoding-safe).
    fixture.match_parse_error(
        &alloc::string::String::from("local \u{2603} = 10"),
        &alloc::string::String::from(
            "Expected identifier when parsing variable name, got Unicode character U+2603",
        ),
        None,
    );
}
