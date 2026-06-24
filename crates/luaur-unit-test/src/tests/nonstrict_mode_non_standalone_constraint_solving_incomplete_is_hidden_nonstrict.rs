//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:336:nonstrict_mode_non_standalone_constraint_solving_incomplete_is_hidden_nonstrict`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CheckedFunctionCallError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item nonstrict_mode_non_standalone_constraint_solving_incomplete_is_hidden_nonstrict

#[cfg(test)]
#[test]
fn nonstrict_mode_non_standalone_constraint_solving_incomplete_is_hidden_nonstrict() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    let _flags = [
        ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
        ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true),
    ];

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let results = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        local function _f(_x: _luau_force_constraint_solving_incomplete) end
        math.abs("pls")
    "#,
        ),
        None,
    );

    assert_eq!(2, results.errors.len(), "{:?}", results.errors);
    assert!(matches!(
        results.errors[0].data,
        TypeErrorData::CheckedFunctionCallError(_)
    ));
    assert!(matches!(
        results.errors[1].data,
        TypeErrorData::ConstraintSolvingIncompleteError(_)
    ));
}
