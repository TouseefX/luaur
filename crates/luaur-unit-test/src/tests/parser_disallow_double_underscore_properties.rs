#[cfg(test)]
#[test]
fn parser_disallow_double_underscore_properties() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
        &alloc::string::String::from(
            "\n        class Foo\n            public __add: any\n        end\n    ",
        ),
        &alloc::string::String::from("Class properties cannot start with '__'"),
        None,
    );
}
