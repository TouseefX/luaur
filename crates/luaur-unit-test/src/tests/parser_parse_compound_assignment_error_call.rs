#[cfg(test)]
#[test]
fn parser_parse_compound_assignment_error_call() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("a() += 5"),
        &alloc::string::String::from("Expected identifier when parsing expression, got '+='"),
        None,
    );
}
