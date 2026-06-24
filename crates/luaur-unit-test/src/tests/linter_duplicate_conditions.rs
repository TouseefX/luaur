//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2237:linter_duplicate_conditions`
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
//!   - translates_to -> rust_item linter_duplicate_conditions

#[cfg(test)]
#[test]
fn linter_duplicate_conditions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
if true then
elseif false then
elseif true then -- duplicate
end

if true then
elseif false then
else
    if true then -- duplicate
    end
end

_ = true and true
_ = true or true
_ = (true and false) and true
_ = (true and true) and true
_ = (true and true) or true
_ = (true and false) and (42 and false)

_ = true and true or false -- no warning since this is is a common pattern used as a ternary replacement

_ = if true then 1 elseif true then 2 else 3
"#,
        ),
        None,
    );

    assert_eq!(8, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Condition has already been checked on line 2",
        result.warnings[0].text.as_str()
    );
    assert_eq!(4, result.warnings[0].location.begin.line + 1);
    assert_eq!(
        "Condition has already been checked on column 5",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "Condition has already been checked on column 5",
        result.warnings[2].text.as_str()
    );
    assert_eq!(
        "Condition has already been checked on column 6",
        result.warnings[3].text.as_str()
    );
    assert_eq!(
        "Condition has already been checked on column 6",
        result.warnings[4].text.as_str()
    );
    assert_eq!(
        "Condition has already been checked on column 6",
        result.warnings[5].text.as_str()
    );
    assert_eq!(
        "Condition has already been checked on column 15",
        result.warnings[6].text.as_str()
    );
    assert_eq!(19, result.warnings[6].location.begin.line + 1);
    assert_eq!(
        "Condition has already been checked on column 8",
        result.warnings[7].text.as_str()
    );
}
