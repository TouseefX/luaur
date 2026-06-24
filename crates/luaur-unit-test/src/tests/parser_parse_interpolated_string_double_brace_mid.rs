#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_double_brace_mid() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    let source = alloc::string::String::from("\n            _ = `{nice} {{oops}}`\n        ");

    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    let errors = result.errors;

    if errors.is_empty() {
        panic!("Expected ParseErrors to be thrown");
    }

    let first_error = errors.first().unwrap();
    let expected_msg =
        "Double braces are not permitted within interpolated strings; did you mean '\\{'?";
    let actual_msg = first_error.get_message();

    if actual_msg.as_str() != expected_msg {
        panic!(
            "Expected error message '{}' but got '{}'",
            expected_msg, actual_msg
        );
    }
}
