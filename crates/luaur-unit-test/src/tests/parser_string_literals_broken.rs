#[cfg(test)]
#[test]
fn parser_string_literals_broken() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("return \""),
        &alloc::string::String::from("Malformed string; did you forget to finish it?"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return \"\\"),
        &alloc::string::String::from("Malformed string; did you forget to finish it?"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("return \"\r\r"),
        &alloc::string::String::from("Malformed string; did you forget to finish it?"),
        None,
    );
}
