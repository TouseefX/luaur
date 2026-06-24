#[cfg(test)]
#[test]
fn parser_classes_cannot_be_shadowed_by_classes() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
        &alloc::string::String::from(
            "\n        class Foobar\n        end\n\n        class Foobar\n        end\n    ",
        ),
        &alloc::string::String::from(
            "A class named 'Foobar' has already been declared in this module",
        ),
        None,
    );
}
