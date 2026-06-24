//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2388:linter_wrong_comment_optimize`
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
//!   - translates_to -> rust_item linter_wrong_comment_optimize

#[cfg(test)]
#[test]
fn linter_wrong_comment_optimize() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let mut result = fixture.lint(
        &String::from(
            r#"
--!optimize
--!optimize me
--!optimize 100500
--!optimize 2
"#,
        ),
        None,
    );

    assert_eq!(3, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "optimize directive requires an optimization level",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "optimize directive uses unknown optimization level 'me', 0..2 expected",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "optimize directive uses unknown optimization level '100500', 0..2 expected",
        result.warnings[2].text.as_str()
    );

    result = fixture.lint(&String::from("--!optimize   "), None);
    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "optimize directive requires an optimization level",
        result.warnings[0].text.as_str()
    );
}
