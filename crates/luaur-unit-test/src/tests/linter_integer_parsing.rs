//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2417:linter_integer_parsing`
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
//!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item linter_integer_parsing

#[cfg(test)]
#[test]
fn linter_integer_parsing() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local _ = 0b10000000000000000000000000000000000000000000000000000000000000000
local _ = 0x10000000000000000
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Binary number literal exceeded available precision and was truncated to 2^64",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Hexadecimal number literal exceeded available precision and was truncated to 2^64",
        result.warnings[1].text.as_str()
    );
}
