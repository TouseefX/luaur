//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:170:nonstrict_mode_table_props_are_any`
//! Source: `tests/NonstrictMode.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/NonstrictMode.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/NonstrictMode.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item nonstrict_mode_table_props_are_any

#[cfg(test)]
#[test]
fn nonstrict_mode_table_props_are_any() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        local T = {}
        T.foo = 55
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let mut opts = ToStringOptions::to_string_options(true);
    let ty = fixture.require_type_string(&String::from("T"));
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "{ foo: number }",
            to_string_type_id_to_string_options(ty, &mut opts)
        );
    } else {
        assert_eq!(
            "{| foo: any |}",
            to_string_type_id_to_string_options(ty, &mut opts)
        );
    }
}
