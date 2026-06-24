#[cfg(test)]
#[test]
fn parser_error_const_reassignment() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag::LuauConst2;
    use luaur_common::FFlag::LuauExportValueSyntax;

    let mut fixture = Fixture::fixture_bool(false);
    let _sff_luau_export_value_syntax = ScopedFastFlag::new(&LuauExportValueSyntax, true);
    let _sff_luau_const2 = ScopedFastFlag::new(&LuauConst2, true);

    fixture.match_parse_error(
        &String::from("const a = 42; a = 43"),
        &String::from("Variable 'a' is constant and may not be reassigned"),
        None,
    );

    fixture.match_parse_error(
        &String::from("local b; const a = 42; a, b = 43"),
        &String::from("Variable 'a' is constant and may not be reassigned"),
        None,
    );

    fixture.match_parse_error(
        &String::from("local b; const a = 42; b, a = 43"),
        &String::from("Variable 'a' is constant and may not be reassigned"),
        None,
    );

    fixture.match_parse_error(
        &String::from("local b; const a = 42; b, a = ..."),
        &String::from("Variable 'a' is constant and may not be reassigned"),
        None,
    );

    fixture.match_parse_error(
        &String::from("const a = 42; function a() end"),
        &String::from("Variable 'a' is constant and may not be reassigned"),
        None,
    );
}
