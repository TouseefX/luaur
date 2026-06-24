//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToString.test.cpp:52:to_string_free_types`
//! Source: `tests/ToString.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ToString.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/ToString.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item to_string_free_types

#[cfg(test)]
#[test]
fn to_string_free_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(&String::from("local a"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "'a",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
}
