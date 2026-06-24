#[cfg(test)]
#[test]
fn parser_duplicate_class_methods() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from(
            "class Hello\n    function hi() end\n    function hi() end\nend",
        ),
        &alloc::string::String::from("Duplicate class member 'hi'"),
        None,
    );
}
