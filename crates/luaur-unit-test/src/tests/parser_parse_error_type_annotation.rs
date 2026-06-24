#[cfg(test)]
#[test]
fn parser_parse_error_type_annotation() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local a : 2 = 2"),
        &alloc::string::String::from("Expected type, got '2'"),
        None,
    );
}
