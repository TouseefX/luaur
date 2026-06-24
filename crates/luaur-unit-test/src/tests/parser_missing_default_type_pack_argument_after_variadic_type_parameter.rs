#[cfg(test)]
#[test]
fn parser_missing_default_type_pack_argument_after_variadic_type_parameter() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("\n        type Foo<T... = > = nil\n    ");

    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 2);

    let expected_location_0 = luaur_ast::records::location::Location::new(
        luaur_ast::records::position::Position::new(1, 23),
        luaur_ast::records::position::Position::new(1, 25),
    );
    assert_eq!(*result.errors[0].get_location(), expected_location_0);
    assert_eq!(*result.errors[0].get_message(), "Expected type, got '>'");

    let expected_location_1 = luaur_ast::records::location::Location::new(
        luaur_ast::records::position::Position::new(1, 23),
        luaur_ast::records::position::Position::new(1, 24),
    );
    assert_eq!(*result.errors[1].get_location(), expected_location_1);
    assert_eq!(
        *result.errors[1].get_message(),
        "Expected type pack after '=', got type"
    );
}
