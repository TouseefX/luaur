#[cfg(test)]
#[test]
fn parser_sense_hot_comment_on_first_line() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::parse_mode::parse_mode;
    use luaur_ast::enums::mode::Mode;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("   --!strict ");
    let mut options = ParseOptions::parse_options();
    options.capture_comments = true;

    let result = fixture.parse_ex(&source, &options);

    let mode = parse_mode(&result.hotcomments);
    assert!(
        mode.is_some(),
        "Expected a mode to be parsed from hotcomments"
    );
    assert_eq!(
        mode.unwrap() as i32,
        Mode::Strict as i32,
        "Mode should be Strict"
    );
}
