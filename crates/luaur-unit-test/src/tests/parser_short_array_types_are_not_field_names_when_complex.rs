#[cfg(test)]
#[test]
fn parser_short_array_types_are_not_field_names_when_complex() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local n: {string | number: number}"),
        &alloc::string::String::from("Expected '}' (to close '{' at column 10), got ':'"),
        None,
    );
}
