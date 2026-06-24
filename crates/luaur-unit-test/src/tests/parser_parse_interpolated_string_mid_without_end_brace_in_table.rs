#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_mid_without_end_brace_in_table() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_error::ParseError;
    use luaur_ast::records::parse_errors::ParseErrors;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("\n            _ = { `x {\"y\"} {z` }\n        ");

    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    let errors = result.errors;
    assert_eq!(2, errors.len());

    let first_error = &errors[0];
    assert_eq!(
        "Malformed interpolated string; did you forget to add a '}'?",
        first_error.get_message().as_str()
    );

    let last_error = &errors[errors.len() - 1];
    assert_eq!(
        "Expected '}' (to close '{' at line 2), got <eof>",
        last_error.get_message().as_str()
    );
}
