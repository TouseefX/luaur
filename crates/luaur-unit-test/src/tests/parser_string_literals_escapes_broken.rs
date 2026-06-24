#[cfg(test)]
#[test]
fn parser_string_literals_escapes_broken() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let expected = "String literal contains malformed escape sequence";

    fixture.match_parse_error(
        &alloc::string::String::from("return \"\\u{\""),
        &alloc::string::String::from(expected),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return \"\\u{FO}\""),
        &alloc::string::String::from(expected),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return \"\\u{123456789}\""),
        &alloc::string::String::from(expected),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return \"\\359\""),
        &alloc::string::String::from(expected),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return \"\\xFO\""),
        &alloc::string::String::from(expected),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return \"\\xF\""),
        &alloc::string::String::from(expected),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return \"\\x\""),
        &alloc::string::String::from(expected),
        None,
    );
}
