#[cfg(test)]
#[test]
fn parser_parse_error_type_name() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local a: Foo.=\n"),
        &alloc::string::String::from("Expected identifier when parsing field name, got '='"),
        None,
    );
}
