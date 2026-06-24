//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2488:linter_comparison_precedence`
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
//!   - translates_to -> rust_item linter_comparison_precedence

#[cfg(test)]
#[test]
fn linter_comparison_precedence() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local a, b = ...

local _ = not a == b
local _ = not a ~= b
local _ = not a <= b
local _ = a <= b == 0
local _ = a <= b <= 0

local _ = not a == not b -- weird but ok

-- silence tests for all of the above
local _ = not (a == b)
local _ = (not a) == b
local _ = not (a ~= b)
local _ = (not a) ~= b
local _ = not (a <= b)
local _ = (not a) <= b
local _ = (a <= b) == 0
local _ = a <= (b == 0)
"#,
        ),
        None,
    );

    assert_eq!(5, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "not X == Y is equivalent to (not X) == Y; consider using X ~= Y, or add parentheses to silence",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "not X ~= Y is equivalent to (not X) ~= Y; consider using X == Y, or add parentheses to silence",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "not X <= Y is equivalent to (not X) <= Y; add parentheses to silence",
        result.warnings[2].text.as_str()
    );
    assert_eq!(
        "X <= Y == Z is equivalent to (X <= Y) == Z; add parentheses to silence",
        result.warnings[3].text.as_str()
    );
    assert_eq!(
        "X <= Y <= Z is equivalent to (X <= Y) <= Z; did you mean X <= Y and Y <= Z?",
        result.warnings[4].text.as_str()
    );
}
