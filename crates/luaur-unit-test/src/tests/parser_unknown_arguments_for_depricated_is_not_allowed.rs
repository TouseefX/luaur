#[cfg(test)]
#[test]
fn parser_unknown_arguments_for_depricated_is_not_allowed() {
    use crate::functions::check_first_error_for_attributes::check_first_error_for_attributes;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();

    let result = fix.try_parse(
        &alloc::string::String::from(
            "\n@[deprecated({}, \"Very deprecated\")]\nfunction hello(x, y)\n    return x + y\nend",
        ),
        &ParseOptions::default(),
    );
    check_first_error_for_attributes(
        &result.errors,
        1,
        Location::new(Position::new(1, 2), Position::new(1, 12)),
        "@deprecated can be parametrized only by 1 argument",
    );

    let result = fix.try_parse(
        &alloc::string::String::from(
            "\n@[deprecated \"Very deprecated\"]\nfunction hello(x, y)\n    return x + y\nend",
        ),
        &ParseOptions::default(),
    );
    check_first_error_for_attributes(
        &result.errors,
        1,
        Location::new(Position::new(1, 13), Position::new(1, 30)),
        "Unknown argument type for @deprecated",
    );

    let result = fix.try_parse(
        &alloc::string::String::from(
            "\n@[deprecated{ foo = \"bar\" }]\nfunction hello(x, y)\n    return x + y\nend",
        ),
        &ParseOptions::default(),
    );
    check_first_error_for_attributes(
        &result.errors,
        1,
        Location::new(Position::new(1, 14), Position::new(1, 17)),
        "Unknown argument 'foo' for @deprecated. Only string constants for 'use' and 'reason' are allowed",
    );

    let result = fix.try_parse(
        &alloc::string::String::from(
            "\n@[deprecated{ use = 5 }]\nfunction hello(x, y)\n    return x + y\nend",
        ),
        &ParseOptions::default(),
    );
    check_first_error_for_attributes(
        &result.errors,
        1,
        Location::new(Position::new(1, 20), Position::new(1, 21)),
        "Only constant string allowed as value for 'use'",
    );
}
