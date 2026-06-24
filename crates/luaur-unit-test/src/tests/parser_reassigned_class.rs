#[cfg(test)]
#[test]
fn parser_reassigned_class() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _g = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);
    let _g_const = ScopedFastFlag::new(&luaur_common::FFlag::LuauConst2, true);
    let _g_export = ScopedFastFlag::new(&luaur_common::FFlag::LuauExportValueSyntax, true);

    let mut fix = Fixture::default();
    fix.match_parse_error(
        &alloc::string::String::from("\nclass Animal end\nAnimal = nil\n        "),
        &alloc::string::String::from("Variable 'Animal' is constant and may not be reassigned"),
        None,
    );
}
