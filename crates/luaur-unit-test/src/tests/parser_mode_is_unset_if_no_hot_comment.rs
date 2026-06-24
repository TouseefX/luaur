#[cfg(test)]
#[test]
fn parser_mode_is_unset_if_no_hot_comment() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from("print('Hello World!')");
    let options = ParseOptions::parse_options();
    let result = fixture.parse_ex(&source, &options);
    assert!(result.hotcomments.is_empty());
}
