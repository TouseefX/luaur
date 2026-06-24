#[cfg(test)]
#[test]
fn parser_invalid_escape_literals_get_reported_but_parsing_continues() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source =
        alloc::string::String::from("\n        local foo = \"\\xQQ\"\n        print(foo)\n    ");
    let options = ParseOptions::parse_options();
    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(1, result.errors.len());

    let expected_location = Location::new(Position::new(1, 20), Position::new(1, 26));
    assert_eq!(&expected_location, result.errors[0].get_location());
    assert_eq!(
        "String literal contains malformed escape sequence",
        *result.errors[0].get_message()
    );

    assert!(!result.root.is_null());
    assert_eq!(2, unsafe { (*result.root).body.size });
}
