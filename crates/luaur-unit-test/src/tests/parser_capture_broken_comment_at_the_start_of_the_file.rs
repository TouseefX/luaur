#[cfg(test)]
#[test]
fn parser_capture_broken_comment_at_the_start_of_the_file() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("\n        --[[\n    ");
    let mut options = ParseOptions::parse_options();
    options.capture_comments = true;

    let result = fix.try_parse(&code, &options);

    assert_eq!(1, result.comment_locations.len());
    let expected_location = Location::new(Position::new(1, 8), Position::new(2, 4));
    assert_eq!(expected_location, result.comment_locations[0].location);
}
