#[cfg(test)]
#[test]
fn parser_nocheck_mode() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::parse_mode::parse_mode;
    use luaur_ast::enums::mode::Mode;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("--!nocheck");
    let mut options = ParseOptions::parse_options();
    options.capture_comments = true;

    let result = fixture.parse_ex(&source, &options);

    assert!(result.errors.is_empty());

    let mode = parse_mode(&result.hotcomments);
    assert!(mode.is_some());

    let mode_val = mode.unwrap();
    assert_eq!(mode_val as i32, Mode::NoCheck as i32);
}
