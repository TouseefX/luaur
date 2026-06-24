#[cfg(test)]
#[test]
fn parser_debugnoinline_not_allowed_without_flag() {
    use crate::functions::check_first_error_for_attributes::check_first_error_for_attributes;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let result = fix.try_parse(
        &alloc::string::String::from(
            "\n@debugnoinline\nlocal function hello(x, y)\n    return x + y\nend",
        ),
        &ParseOptions::default(),
    );

    check_first_error_for_attributes(
        &result.errors,
        1,
        Location::new(Position::new(1, 0), Position::new(1, 14)),
        "Invalid attribute '@debugnoinline'",
    );
}
