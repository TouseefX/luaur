//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_checked_fn_to_string() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::enums::mode::Mode;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    fixture.load_definition(
        &String::from(
            r#"
@checked declare function abs(n: number) : number
"#,
        ),
        false,
    );

    let result = fixture.check_mode_string_optional_frontend_options(
        Mode::Nonstrict,
        &String::from(
            r#"
local f = abs
"#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "@checked (number) -> number",
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
}
