#[cfg(test)]
#[test]
fn parser_classes_can_have_members_named_public() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::parse_options::ParseOptions;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    let src = alloc::string::String::from(
        "\n        class Foobar\n            function public() end\n        end\n\n        class Barbaz\n            public public\n        end\n    ",
    );
    let result = fix.try_parse(&src, &ParseOptions::default());
    assert!(result.errors.is_empty());
}
