#[cfg(test)]
#[test]
fn parser_overlapping_property_and_method_names() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
        &alloc::string::String::from(
            "\nclass Hello\n    public helloagain\n    function helloagain() end\nend\n        ",
        ),
        &alloc::string::String::from("Duplicate class member 'helloagain'"),
        None,
    );
}
