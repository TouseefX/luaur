#[cfg(test)]
#[test]
fn parser_classes_can_be_shadowed_by_locals() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::parse_options::ParseOptions;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    // This is legal: exactly one class with a given name, but it may be shadowed by a local.
    let src = alloc::string::String::from(
        "\n        class Foobar\n        end\n\n        -- This is legal: the rule is that there is exactly one class with a\n        -- given name, but we can shadow it with a local.\n        local Foobar\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(result.errors.is_empty());
}
