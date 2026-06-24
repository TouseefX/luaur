//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2465:linter_integer_parsing_hex_imprecise`
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
//!   - calls -> method AssemblyBuilderA64::bit (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item linter_integer_parsing_hex_imprecise

#[cfg(test)]
#[test]
fn linter_integer_parsing_hex_imprecise() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local _ = 0x1234567812345678

-- smallest possible number that is parsed imprecisely
local _ = 0x20000000000001

-- note that numbers before and after parse precisely (number after is even => 1 more mantissa bit)
local _ = 0x20000000000000
local _ = 0x20000000000002

-- large powers of two should work as well (this is 2^63)
local _ = 0x80000000000000
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Number literal exceeded available precision and was truncated to closest representable number",
        result.warnings[0].text.as_str()
    );
    assert_eq!(1, result.warnings[0].location.begin.line);
    assert_eq!(
        "Number literal exceeded available precision and was truncated to closest representable number",
        result.warnings[1].text.as_str()
    );
    assert_eq!(4, result.warnings[1].location.begin.line);
}
