#[cfg(test)]
#[test]
fn parser_unfinished_string_literal_types_get_reported_but_parsing_continues() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("\n        type Foo = \"hi\n        print(foo)\n    ");
    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(1, result.errors.len());

    let expected_location = luaur_ast::records::location::Location::new(
        luaur_ast::records::position::Position::new(1, 19),
        luaur_ast::records::position::Position::new(1, 22),
    );
    assert_eq!(expected_location, *result.errors[0].get_location());
    assert_eq!(
        "Malformed string; did you forget to finish it?",
        result.errors[0].get_message()
    );

    assert!(result.root.is_null() == false);
    assert_eq!(2, unsafe { (*result.root).body.size });
}
