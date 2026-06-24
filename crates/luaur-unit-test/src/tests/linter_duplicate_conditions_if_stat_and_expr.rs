//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2375:linter_duplicate_conditions_if_stat_and_expr`
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
//!   - translates_to -> rust_item linter_duplicate_conditions_if_stat_and_expr

#[cfg(test)]
#[test]
fn linter_duplicate_conditions_if_stat_and_expr() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
if if 1 then 2 else 3 then
elseif if 1 then 2 else 3 then
elseif if 0 then 5 else 4 then
end
"#,
        ),
        None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Condition has already been checked on line 2",
        result.warnings[0].text.as_str()
    );
}
