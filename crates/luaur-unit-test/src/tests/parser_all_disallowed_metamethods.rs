#[cfg(test)]
#[test]
fn parser_all_disallowed_metamethods() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let code = r#"class Foo
    function __index() end
    function __newindex() end
    function __mode() end
    function __metatable() end
    function __type() end
    function __doesnotexist() end
end"#;

    let result = fix.try_parse(&code.to_string(), &ParseOptions::default());

    assert_eq!(result.errors.len(), 6);
    assert_eq!(
        result.errors[0].get_message(),
        "Classes cannot define '__index' as a metamethod"
    );
    assert_eq!(
        result.errors[1].get_message(),
        "Classes cannot define '__newindex' as a metamethod"
    );
    assert_eq!(
        result.errors[2].get_message(),
        "Classes cannot define '__mode' as a metamethod"
    );
    assert_eq!(
        result.errors[3].get_message(),
        "Classes cannot define '__metatable' as a metamethod"
    );
    assert_eq!(
        result.errors[4].get_message(),
        "Classes cannot define '__type' as a metamethod"
    );
    assert_eq!(
        result.errors[5].get_message(),
        "Cannot use '__doesnotexist' as a method name: names starting with '__' are reserved"
    );
}
