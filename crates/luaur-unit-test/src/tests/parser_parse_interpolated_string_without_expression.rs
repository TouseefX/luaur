#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_without_expression() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_ast::records::parse_error::ParseError;
    use luaur_ast::records::parse_errors::ParseErrors;

    let mut fixture = Fixture::fixture_bool(false);

    // C++ catches a thrown ParseErrors and checks the first message. try_parse does
    // error recovery and returns a (partial) non-null root, so gating on
    // root.is_null() is wrong — just assert the error directly.
    let source1 = String::from("print(`{}`)");
    let parse_result1 = fixture.try_parse(
        &source1,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );
    assert!(
        !parse_result1.errors.is_empty(),
        "Expected ParseErrors to be thrown"
    );
    assert_eq!(
        "Malformed interpolated string, expected expression inside '{}'",
        parse_result1.errors[0].get_message().as_str()
    );

    let source2 = String::from("print(`{}{1}`)");
    let parse_result2 = fixture.try_parse(
        &source2,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );
    assert!(
        !parse_result2.errors.is_empty(),
        "Expected ParseErrors to be thrown"
    );
    assert_eq!(
        "Malformed interpolated string, expected expression inside '{}'",
        parse_result2.errors[0].get_message().as_str()
    );
}
