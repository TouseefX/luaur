#[cfg(test)]
#[test]
fn parser_dont_parse_attribute_on_argument_non_function() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\nlocal function invoker(f, y)\n    return f(y)\nend\n\ninvoker(function(x) return (x + 2) end, @checked 1)\n",
    );
    let options = ParseOptions::parse_options();
    let result = fixture.try_parse(&source, &options);

    let expected_location = Location::new(Position::new(5, 40), Position::new(5, 48));
    let expected_message = "Expected 'function' declaration after attribute, but got '1' instead";

    crate::functions::check_first_error_for_attributes::check_first_error_for_attributes(
        &result.errors,
        1,
        expected_location,
        expected_message,
    );
}
