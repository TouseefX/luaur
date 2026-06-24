#[cfg(test)]
#[test]
fn parser_error_const_function_reassignment() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauConst2;
    use luaur_common::FFlag::LuauExportValueSyntax;

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);
    let _sff_luau_const2 = ScopedFastFlag::new(&LuauConst2, true);

    let source = alloc::string::String::from("const function a() return 42 end; a = 43");
    let message = alloc::string::String::from("Variable 'a' is constant and may not be reassigned");

    let _result = fixture.match_parse_error(&source, &message, None);
}
