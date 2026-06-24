#[cfg(test)]
#[test]
fn parser_nil_can_not_be_a_field_name() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local n: {nil: number}"),
        &alloc::string::String::from("Expected '}' (to close '{' at column 10), got ':'"),
        None,
    );
}
