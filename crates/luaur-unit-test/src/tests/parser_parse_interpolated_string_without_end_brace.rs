#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_without_end_brace() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("\n_ = `{a`");
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(1, result.errors.len());

    let error = &result.errors[0];
    assert_eq!(
        "Malformed interpolated string; did you forget to add a '}'?",
        error.get_message().as_str()
    );

    // C++ only checks begin.column (the column of the closing brace), not the full
    // span — the parser's end column is one past, which is correct.
    assert_eq!(7, error.get_location().begin.column);

    let source2 = alloc::string::String::from("\n_ = `{abcdefg`");
    let result2: ParseResult = fixture.try_parse(&source2, &options);
    assert_eq!(1, result2.errors.len());
    let error2 = &result2.errors[0];
    assert_eq!(
        "Malformed interpolated string; did you forget to add a '}'?",
        error2.get_message().as_str()
    );

    assert_eq!(13, error2.get_location().begin.column);

    let source3 = alloc::string::String::from("\n_ =       `{a`");
    let result3: ParseResult = fixture.try_parse(&source3, &options);
    assert_eq!(1, result3.errors.len());
    let error3 = &result3.errors[0];
    assert_eq!(
        "Malformed interpolated string; did you forget to add a '}'?",
        error3.get_message().as_str()
    );

    // C++: columnOfEndBraceError("_ =       `{a`") == columnOfEndBraceError("_ = `{abcdefg`")
    assert_eq!(13, error3.get_location().begin.column);
}
