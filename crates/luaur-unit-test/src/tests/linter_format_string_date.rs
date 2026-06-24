//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1170:linter_format_string_date`
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
//!   - translates_to -> rust_item linter_format_string_date

#[cfg(test)]
#[test]
fn linter_format_string_date() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
-- incorrect formats
os.date("%")
os.date("%L")
os.date("%?")
os.date("\0")

-- correct formats
os.date("it's %c now")
os.date("!*t")
"#,
        ),
        None,
    );

    assert_eq!(4, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid date format: unfinished replacement",
        result.warnings[0].text
    );
    assert_eq!(
        "Invalid date format: unexpected replacement character; must be a date format specifier or %",
        result.warnings[1].text
    );
    assert_eq!(
        "Invalid date format: unexpected replacement character; must be a date format specifier or %",
        result.warnings[2].text
    );
    assert_eq!(
        "Invalid date format: date format can not contain null characters",
        result.warnings[3].text
    );
}
