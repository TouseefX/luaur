#[cfg(test)]
#[test]
fn parser_capture_broken_comment() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::default();
    let source =
        alloc::string::String::from("\n        local a = \"test\"\n\n        --[[broken!\n    ");
    let mut options = ParseOptions::parse_options();
    options.capture_comments = true;

    let result: ParseResult = fixture.try_parse(&source, &options);

    assert_eq!(result.comment_locations.len(), 1);
    let expected_location = Location::new(Position::new(3, 8), Position::new(4, 4));
    assert_eq!(result.comment_locations[0].location, expected_location);
}
