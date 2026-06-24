#[cfg(test)]
#[test]
fn parser_duplicate_unnamed_class_methods() {
    use crate::records::fixture::Fixture;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _scoped_flag = luaur_common::FFlag::DebugLuauUserDefinedClasses.set(true);

    let mut fixture = Fixture::default();
    let source = "\nclass Hello\n    function () end\n    function () end\nend\n        ";

    let result = fixture.try_parse(
        &source.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert_eq!(result.errors.len(), 3);
    assert_eq!(
        result.errors[0].get_message(),
        "Expected identifier when parsing method name, got '('"
    );
    assert_eq!(
        result.errors[1].get_message(),
        "Expected identifier when parsing method name, got '('"
    );
    assert_eq!(
        result.errors[2].get_message(),
        "Duplicate class member '%error-id%'"
    );
}
