//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1078:linter_format_string_match_sets`
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
//!   - calls -> method TxnLog::inverse (Analysis/src/TxnLog.cpp)
//!   - translates_to -> rust_item linter_format_string_match_sets

#[cfg(test)]
#[test]
fn linter_format_string_match_sets() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local s = ...

-- fake empty sets (but actually sets that aren't closed)
string.match(s, "[]")
string.match(s, "[^]")

-- character ranges in sets
string.match(s, "[%a-b]")
string.match(s, "[a-%b]")

-- invalid escapes
string.match(s, "[%q]")
string.match(s, "[%;]")

-- capture refs in sets
string.match(s, "[%1]")

-- valid escapes and - at the end
string.match(s, "[%]x-]")

-- % escapes itself
string.match(s, "[%%]")

-- this abomination is a valid pattern due to rules wrt handling empty sets
string.match(s, "[]|'[]")
string.match(s, "[^]|'[]")
"#,
        ),
        None,
    );

    assert_eq!(7, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid match pattern: expected ] at the end of the string to close a set",
        result.warnings[0].text
    );
    assert_eq!(
        "Invalid match pattern: expected ] at the end of the string to close a set",
        result.warnings[1].text
    );
    assert_eq!(
        "Invalid match pattern: character range can't include character sets",
        result.warnings[2].text
    );
    assert_eq!(
        "Invalid match pattern: character range can't include character sets",
        result.warnings[3].text
    );
    assert_eq!(
        "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
        result.warnings[4].text
    );
    assert_eq!(
        "Invalid match pattern: expected a magic character after %",
        result.warnings[5].text
    );
    assert_eq!(
        "Invalid match pattern: sets can not contain capture references",
        result.warnings[6].text
    );
}
