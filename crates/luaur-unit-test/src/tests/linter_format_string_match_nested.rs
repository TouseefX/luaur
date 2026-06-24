//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1056:linter_format_string_match_nested`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function match (VM/src/lstrlib.cpp)
//!   - translates_to -> rust_item linter_format_string_match_nested

#[cfg(test)]
#[test]
fn linter_format_string_match_nested() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local s = ...

-- correct reference to nested pattern
string.match(s, "((a)%2)")

-- incorrect reference to nested pattern (not closed yet)
string.match(s, "((a)%1)")

-- incorrect reference to nested pattern (index out of range)
string.match(s, "((a)%3)")
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid match pattern: invalid capture reference, must refer to a closed capture",
        result.warnings[0].text
    );
    assert_eq!(7, result.warnings[0].location.begin.line);
    assert_eq!(
        "Invalid match pattern: invalid capture reference, must refer to a valid capture",
        result.warnings[1].text
    );
    assert_eq!(10, result.warnings[1].location.begin.line);
}
