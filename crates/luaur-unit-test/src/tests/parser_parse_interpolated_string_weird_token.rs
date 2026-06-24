#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_weird_token() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("\n            local a = `??? {42 !!}`\n        "),
        // C++ expects no trailing period; the port added one.
        &alloc::string::String::from("Malformed interpolated string, got '!'"),
        None,
    );
}
