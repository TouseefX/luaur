#[cfg(test)]
#[test]
fn parser_classes_nested_and_repeated() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_common::FFlag;

    let _debug_luau_user_defined_classes =
        ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let code = r#"
        class Foo
        end
        if true then
            class Foo
            end
        end
    "#;

    let result = fix.try_parse(&code.to_string(), &ParseOptions::default());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        &*result.errors[0].get_message(),
        "Cannot declare class 'Foo' inside another statement or expression"
    );
}
