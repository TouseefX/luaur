#[cfg(test)]
#[test]
fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_function_parameter_list() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\n        export type VisitFn = (\n            any,\n            Array<TAnyNode | Array<TAnyNode>>, -- extra comma here\n        ) -> any\n    ",
    );
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let error = &result.errors[0];
    let expected_location = Location::new(Position::new(4, 8), Position::new(4, 9));
    assert_eq!(*error.get_location(), expected_location);
    assert_eq!(
        error.get_message().as_str(),
        "Expected type after ',' but got ')' instead"
    );
}
