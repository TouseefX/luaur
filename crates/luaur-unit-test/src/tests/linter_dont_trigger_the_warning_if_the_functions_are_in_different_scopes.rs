//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1466:linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes`
//! Source: `tests/Linter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Linter.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Linter.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Linter.test.cpp
//! - outgoing:
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - translates_to -> rust_item linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes

#[cfg(test)]
#[test]
fn linter_dont_trigger_the_warning_if_the_functions_are_in_different_scopes() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
        if true then
            function c() end
        else
            function c() end
        end

        return c
    "#,
        ),
        None,
    );

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}
