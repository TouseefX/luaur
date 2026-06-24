//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:962:linter_format_string_pack`
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
//!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> method StringWriter::space (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item linter_format_string_pack

#[cfg(test)]
#[test]
fn linter_format_string_pack() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
-- incorrect pack specifiers
string.pack("?")
string.packsize("?")
string.unpack("?")

-- missing size
string.packsize("bc")

-- incorrect X alignment
string.packsize("X")
string.packsize("X i")

-- correct X alignment
string.packsize("Xi")

-- packsize can't be used with variable sized formats
string.packsize("s")

-- out of range size specifiers
string.packsize("i0")
string.packsize("i17")

-- a very very very out of range size specifier
string.packsize("i99999999999999999999")
string.packsize("c99999999999999999999")

-- correct format specifiers
string.packsize("=!1bbbI3c42")
"#,
        ),
        None,
    );

    assert_eq!(11, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid pack format: unexpected character; must be a pack specifier or space",
        result.warnings[0].text
    );
    assert_eq!(
        "Invalid pack format: unexpected character; must be a pack specifier or space",
        result.warnings[1].text
    );
    assert_eq!(
        "Invalid pack format: unexpected character; must be a pack specifier or space",
        result.warnings[2].text
    );
    assert_eq!(
        "Invalid pack format: fixed-sized string format must specify the size",
        result.warnings[3].text
    );
    assert_eq!(
        "Invalid pack format: X must be followed by a size specifier",
        result.warnings[4].text
    );
    assert_eq!(
        "Invalid pack format: X must be followed by a size specifier",
        result.warnings[5].text
    );
    assert_eq!(
        "Invalid pack format: pack specifier must be fixed-size",
        result.warnings[6].text
    );
    assert_eq!(
        "Invalid pack format: integer size must be in range [1,16]",
        result.warnings[7].text
    );
    assert_eq!(
        "Invalid pack format: integer size must be in range [1,16]",
        result.warnings[8].text
    );
    assert_eq!(
        "Invalid pack format: size specifier is too large",
        result.warnings[9].text
    );
    assert_eq!(
        "Invalid pack format: size specifier is too large",
        result.warnings[10].text
    );
}
