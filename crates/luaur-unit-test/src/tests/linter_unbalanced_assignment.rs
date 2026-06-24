//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:720:linter_unbalanced_assignment`
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
//!   - translates_to -> rust_item linter_unbalanced_assignment

#[cfg(test)]
#[test]
fn linter_unbalanced_assignment() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
do
local _a,_b,_c = pcall()
end
do
local _a,_b,_c = pcall(), 5
end
do
local _a,_b,_c = pcall(), 5, 6
end
do
local _a,_b,_c = pcall(), 5, 6, 7
end
do
local _a,_b,_c = pcall(), nil
end
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(5, result.warnings[0].location.begin.line);
    assert_eq!(
        "Assigning 2 values to 3 variables initializes extra variables with nil; add 'nil' to value list to silence",
        result.warnings[0].text
    );
    assert_eq!(11, result.warnings[1].location.begin.line);
    assert_eq!(
        "Assigning 4 values to 3 variables leaves some values unused",
        result.warnings[1].text
    );
}
