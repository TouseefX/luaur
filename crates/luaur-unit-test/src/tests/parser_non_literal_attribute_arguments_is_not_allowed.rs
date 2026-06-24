#[cfg(test)]
#[test]
fn parser_non_literal_attribute_arguments_is_not_allowed() {
    use crate::functions::check_first_error_for_attributes::check_first_error_for_attributes;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "\n@[deprecated{ reason = reasonString }]\nfunction hello(x, y)\n    return x + y\nend",
    );

    let result = fix.try_parse(
        &code,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    let expected_location = Location::new(Position::new(1, 13), Position::new(1, 37));
    let expected_message = "Only literals can be passed as arguments for attributes";

    check_first_error_for_attributes(&result.errors, 1, expected_location, expected_message);
}
