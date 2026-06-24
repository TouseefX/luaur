#[cfg(test)]
#[test]
fn parser_class_self_cannot_be_annotated() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
        &alloc::string::String::from(
            "\n        class Foobar\n            function baz(self: number, foobar)\n        end\n    ",
        ),
        &alloc::string::String::from("The 'self' parameter cannot have a type annotation"),
        None,
    );
}
