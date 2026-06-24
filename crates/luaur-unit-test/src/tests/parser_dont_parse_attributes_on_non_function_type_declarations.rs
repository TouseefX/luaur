#[cfg(test)]
#[test]
fn parser_dont_parse_attributes_on_non_function_type_declarations() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;

    let source1 = alloc::string::String::from("\n@checked declare foo: number\n");
    let result1 = fixture.try_parse(&source1, &opts);

    let expected_location1 = Location::new(Position::new(1, 17), Position::new(1, 20));
    let expected_message1 =
        "Expected a function type declaration after attribute, but got 'foo' instead";
    crate::functions::check_first_error_for_attributes::check_first_error_for_attributes(
        &result1.errors,
        1,
        expected_location1,
        expected_message1,
    );

    let source2 = alloc::string::String::from(
        "\n@checked declare class Foo\n    prop: number\n    function method(self, foo: number): string\nend)",
    );
    let result2 = fixture.try_parse(&source2, &opts);

    let expected_location2 = Location::new(Position::new(1, 17), Position::new(1, 22));
    let expected_message2 =
        "Expected a function type declaration after attribute, but got 'class' instead";
    crate::functions::check_first_error_for_attributes::check_first_error_for_attributes(
        &result2.errors,
        1,
        expected_location2,
        expected_message2,
    );

    let source3 = alloc::string::String::from("\ndeclare bit32: {\n    band: @checked number\n})");
    let result3 = fixture.try_parse(&source3, &opts);

    let expected_location3 = Location::new(Position::new(2, 19), Position::new(2, 25));
    let expected_message3 = "Expected '(' when parsing function parameters, got 'number'";
    crate::functions::check_first_error_for_attributes::check_first_error_for_attributes(
        &result3.errors,
        1,
        expected_location3,
        expected_message3,
    );
}
