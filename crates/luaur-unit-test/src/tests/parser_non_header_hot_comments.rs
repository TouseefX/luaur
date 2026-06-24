#[cfg(test)]
#[test]
fn parser_non_header_hot_comments() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::parse_mode::parse_mode;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("do end --!strict");
    let mut options = ParseOptions::parse_options();
    options.capture_comments = true;
    let result = fixture.parse_ex(&source, &options);
    let mode = parse_mode(&result.hotcomments);
    assert!(mode.is_none());
}
