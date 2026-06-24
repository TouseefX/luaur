//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1147:linter_format_string_replace`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function match (VM/src/lstrlib.cpp)
//!   - calls -> function digit (VM/src/lstrlib.cpp)
//!   - translates_to -> rust_item linter_format_string_replace

#[cfg(test)]
#[test]
fn linter_format_string_replace() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local s = ...

-- incorrect replacements
string.gsub(s, '(%d+)', "%")
string.gsub(s, '(%d+)', "%x")
string.gsub(s, '(%d+)', "%2")
string.gsub(s, '', "%1")

-- correct replacements
string.gsub(s, '[A-Z]+(%d)', "%0%1")
string.gsub(s, 'foo', "%0")
"#,
        ),
        None,
    );

    assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid match replacement: unfinished replacement",
        result.warnings[0].text
    );
    assert_eq!(
        "Invalid match replacement: unexpected replacement character; must be a digit or %",
        result.warnings[1].text
    );
    assert_eq!(
        "Invalid match replacement: invalid capture index, must refer to pattern capture",
        result.warnings[2].text
    );
    assert_eq!(
        "Invalid match replacement: invalid capture index, must refer to pattern capture",
        result.warnings[3].text
    );
}
