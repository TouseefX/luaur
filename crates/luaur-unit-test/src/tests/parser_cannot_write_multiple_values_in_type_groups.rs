#[cfg(test)]
#[test]
fn parser_cannot_write_multiple_values_in_type_groups() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("type F = ((string, number))"),
        &alloc::string::String::from("Expected '->' when parsing function type, got ')'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("type F = () -> ((string, number))"),
        &alloc::string::String::from("Expected '->' when parsing function type, got ')'"),
        None,
    );
}
