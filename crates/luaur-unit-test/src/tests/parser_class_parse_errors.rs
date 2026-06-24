#[cfg(test)]
#[test]
fn parser_class_parse_errors() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::parse_options::ParseOptions;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let mut fix = Fixture::default();

    // C++ only checks that the parser does not crash on these malformed inputs.
    let opts = ParseOptions::default();
    fix.try_parse(&alloc::string::String::from(" class Hello "), &opts);
    fix.try_parse(&alloc::string::String::from(" class Hello public "), &opts);
    fix.try_parse(
        &alloc::string::String::from(" class Hello public x "),
        &opts,
    );
    fix.try_parse(
        &alloc::string::String::from(" class Hello public x: "),
        &opts,
    );
    fix.try_parse(
        &alloc::string::String::from(" class Hello public x: number "),
        &opts,
    );
    fix.try_parse(&alloc::string::String::from(" class Hello end "), &opts);
    fix.try_parse(
        &alloc::string::String::from(" class Hello public end "),
        &opts,
    );
    fix.try_parse(
        &alloc::string::String::from(" class Hello private end "),
        &opts,
    );
    fix.try_parse(
        &alloc::string::String::from(" class Hello public x end "),
        &opts,
    );
    fix.try_parse(
        &alloc::string::String::from(" class Hello public x: end "),
        &opts,
    );
    fix.try_parse(
        &alloc::string::String::from(" class Hello public x: number end "),
        &opts,
    );
    fix.try_parse(
        &alloc::string::String::from(" class Hello public x: number public x: string end "),
        &opts,
    );
    fix.try_parse(
        &alloc::string::String::from(" class Hello public x: number function x() end end "),
        &opts,
    );
}
