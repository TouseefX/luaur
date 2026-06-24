//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1191:linter_format_string_typed`
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
//!   - translates_to -> rust_item linter_format_string_typed

#[cfg(test)]
#[test]
fn linter_format_string_typed() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local s: string, nons = ...

string.match(s, "[]")
s:match("[]")

-- no warning here since we don't know that it's a string
nons:match("[]")
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid match pattern: expected ] at the end of the string to close a set",
        result.warnings[0].text.as_str()
    );
    assert_eq!(3, result.warnings[0].location.begin.line);
    assert_eq!(
        "Invalid match pattern: expected ] at the end of the string to close a set",
        result.warnings[1].text.as_str()
    );
    assert_eq!(4, result.warnings[1].location.begin.line);
}
