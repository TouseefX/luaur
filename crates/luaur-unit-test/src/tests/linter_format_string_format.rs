//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:940:linter_format_string_format`
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
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item linter_format_string_format

#[cfg(test)]
#[test]
fn linter_format_string_format() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
-- incorrect format strings
string.format("%")
string.format("%??d")
string.format("%Y")

-- incorrect format strings, self call
local _ = ("%"):format()

-- correct format strings, just to uh make sure
string.format("hello %+10d %.02f %%", 4, 5)
"#,
        ),
        None,
    );

    assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid format string: unfinished format specifier",
        result.warnings[0].text
    );
    assert_eq!(
        "Invalid format string: invalid format specifier: must be a string format specifier or %",
        result.warnings[1].text
    );
    assert_eq!(
        "Invalid format string: invalid format specifier: must be a string format specifier or %",
        result.warnings[2].text
    );
    assert_eq!(
        "Invalid format string: unfinished format specifier",
        result.warnings[3].text
    );
}
