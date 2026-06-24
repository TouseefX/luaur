#[cfg(test)]
#[test]
fn parser_capture_comments() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "\n        --!strict\n\n        local a = 5 -- comment one\n        local b = 8 -- comment two\n        --[[\n            Multi line comment\n        ]]\n        local c = 'see'\n    ",
    );
    let mut options = ParseOptions::parse_options();
    options.capture_comments = true;

    let result = fixture.parse_ex(&source, &options);

    assert!(result.errors.is_empty());
    assert_eq!(result.comment_locations.len(), 4);

    let loc0 = Location::new(Position::new(1, 8), Position::new(1, 17));
    assert_eq!(result.comment_locations[0].location, loc0);

    let loc1 = Location::new(Position::new(3, 20), Position::new(3, 34));
    assert_eq!(result.comment_locations[1].location, loc1);

    let loc2 = Location::new(Position::new(4, 20), Position::new(4, 34));
    assert_eq!(result.comment_locations[2].location, loc2);

    let loc3 = Location::new(Position::new(5, 8), Position::new(7, 10));
    assert_eq!(result.comment_locations[3].location, loc3);
}
