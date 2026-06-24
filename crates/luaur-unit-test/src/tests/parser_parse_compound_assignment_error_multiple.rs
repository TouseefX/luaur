#[cfg(test)]
#[test]
fn parser_parse_compound_assignment_error_multiple() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = alloc::string::String::from("a, b += 5");
    // C++ catches the thrown ParseErrors. `parse_ex` PANICS on errors, so use
    // try_parse (returns the errors) and inspect them.
    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    // Check that parsing failed with ParseErrors
    if result.errors.is_empty() {
        panic!("Expected ParseErrors to be thrown");
    }

    let errors = &result.errors;
    let error_vec = errors;

    // Check that we have at least one error
    if error_vec.is_empty() {
        panic!("Expected at least one parse error");
    }

    // Get the first error's message
    let first_error = &error_vec[0];
    let actual_message = first_error.get_message();
    let expected_message =
        alloc::string::String::from("Expected '=' when parsing assignment, got '+='");

    if actual_message != &expected_message {
        panic!(
            "Expected error message '{}' but got '{}'",
            expected_message, actual_message
        );
    }
}
