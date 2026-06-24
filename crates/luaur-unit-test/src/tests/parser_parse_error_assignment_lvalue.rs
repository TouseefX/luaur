#[cfg(test)]
#[test]
fn parser_parse_error_assignment_lvalue() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local a, b\n(2), b = b, a\n"),
        &alloc::string::String::from("Assigned expression must be a variable or a field"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("local a, b\na, (3) = b, a\n"),
        &alloc::string::String::from("Assigned expression must be a variable or a field"),
        None,
    );
}
