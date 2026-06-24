#[cfg(test)]
#[test]
fn parser_table_type_keys_cant_contain_nul() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("\n        type Foo = { [\"\\0\"]: number }\n    ");
    let result = fix.try_parse(
        &code,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert_eq!(1, result.errors.len());

    let error = &result.errors[0];
    let expected_location = Location::new(Position::new(1, 21), Position::new(1, 22));
    assert_eq!(expected_location, *error.get_location());
    assert_eq!(
        "String literal contains malformed escape sequence or \\0",
        *error.get_message()
    );
}
