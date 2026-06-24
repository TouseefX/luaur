//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:697:linter_for_range_zero`
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
//!   - translates_to -> rust_item linter_for_range_zero

#[cfg(test)]
#[test]
fn linter_for_range_zero() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
for i=0,#t do
end

for i=(0),#t do -- to silence
end

for i=#t,0 do
end
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(1, result.warnings[0].location.begin.line);
    assert_eq!(
        "For loop starts at 0, but arrays start at 1",
        result.warnings[0].text
    );
    assert_eq!(7, result.warnings[1].location.begin.line);
    assert_eq!(
        "For loop should iterate backwards; did you forget to specify -1 as step? Also consider changing 0 to 1 since arrays start at 1",
        result.warnings[1].text
    );
}
