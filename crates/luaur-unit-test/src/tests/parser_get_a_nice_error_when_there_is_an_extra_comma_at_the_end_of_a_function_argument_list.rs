#[cfg(test)]
#[test]
fn parser_get_a_nice_error_when_there_is_an_extra_comma_at_the_end_of_a_function_argument_list() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("\n        foo(a, b, c,)\n    ");
    let options = ParseOptions::parse_options();
    let result = fixture.try_parse(&source, &options);

    assert_eq!(result.errors.len(), 1);

    let expected_location = Location::new(Position::new(1, 20), Position::new(1, 21));
    assert_eq!(*result.errors[0].get_location(), expected_location);

    let expected_message = "Expected expression after ',' but got ')' instead";
    assert_eq!(&*result.errors[0].get_message(), expected_message);
}
