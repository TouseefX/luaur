#[cfg(test)]
#[test]
fn parser_empty_attribute_name_is_not_allowed() {
    use crate::functions::check_first_error_for_attributes::check_first_error_for_attributes;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let code = "\n@\nfunction hello(x, y)\n    return x + y\nend";

    let result = fix.try_parse(
        &code.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    let expected_location = Location::new(Position::new(1, 0), Position::new(1, 1));
    let expected_message = "Attribute name is missing";

    check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
}
