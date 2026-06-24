#[cfg(test)]
#[test]
fn parser_parse_error_function_call_newline() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "\nfunction stringifyTable(t)\n    local foo = t:Parse\n    return foo\nend\n        ",
    );
    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 1);
    let first_error = &result.errors[0];
    assert_eq!(first_error.get_location().begin.line, 2);
    assert_eq!(
        &*first_error.get_message(),
        "Expected function call arguments after '('"
    );
}
