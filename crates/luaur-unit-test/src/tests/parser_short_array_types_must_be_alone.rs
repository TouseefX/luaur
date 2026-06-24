#[cfg(test)]
#[test]
fn parser_short_array_types_must_be_alone() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local n: {string, number}"),
        &alloc::string::String::from("Expected '}' (to close '{' at column 10), got ','"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local n: {[number]: string, number}"),
        &alloc::string::String::from("Expected ':' when parsing table field, got '}'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local n: {x: string, number}"),
        &alloc::string::String::from("Expected ':' when parsing table field, got '}'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local n: {x: string, nil}"),
        &alloc::string::String::from("Expected identifier when parsing table field, got 'nil'"),
        None,
    );
}
