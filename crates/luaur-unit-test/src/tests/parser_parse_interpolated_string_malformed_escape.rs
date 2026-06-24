#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_malformed_escape() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected = "Interpolated string literal contains malformed escape sequence";

    fixture.match_parse_error(
        &alloc::string::String::from("local a = `???\\xQQ {1}`"),
        &alloc::string::String::from(expected),
        None,
    );
}
