#[cfg(test)]
#[test]
fn parser_type_alias_error_messages() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("type 5 = number"),
        &alloc::string::String::from("Expected identifier when parsing type name, got '5'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type A"),
        &alloc::string::String::from("Expected '=' when parsing type alias, got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type A<"),
        &alloc::string::String::from("Expected identifier, got <eof>"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type A<B"),
        &alloc::string::String::from("Expected '>' (to close '<' at column 7), got <eof>"),
        None,
    );
}
