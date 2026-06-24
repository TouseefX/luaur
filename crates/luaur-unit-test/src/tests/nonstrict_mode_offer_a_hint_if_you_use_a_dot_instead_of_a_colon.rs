//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/NonstrictMode.test.cpp:155:nonstrict_mode_offer_a_hint_if_you_use_a_dot_instead_of_a_colon`
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
//!   - translates_to -> rust_item nonstrict_mode_offer_a_hint_if_you_use_a_dot_instead_of_a_colon

#[cfg(test)]
#[test]
fn nonstrict_mode_offer_a_hint_if_you_use_a_dot_instead_of_a_colon() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = Fixture::fixture_bool(false);
    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        local T = {}
        function T:method(x: number) end
        T.method(5)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "This function must be called with self. Did you mean to use a colon instead of a dot?",
        to_string_type_error(&result.errors[0])
    );
}
