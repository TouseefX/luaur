#[cfg(test)]
#[test]
fn parser_class_method_missing_end_error() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from(
            "\n        class Foo\n            function bar()\n                local x = 1\n    ",
        ),
        &alloc::string::String::from("Expected 'end' (to close 'function' at line 3), got <eof>"),
        None,
    );
}
