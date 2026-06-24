//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:40:linter_unknown_global`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item linter_unknown_global

#[cfg(test)]
#[test]
fn linter_unknown_global() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(&String::from("--!nocheck\nreturn foo"), None);

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Unknown global 'foo'; consider assigning to it first",
        result.warnings[0].text.as_str()
    );
}
