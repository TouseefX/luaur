//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1009:linter_format_string_match`
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
//!   - calls -> function gmatch (VM/src/lstrlib.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method TxnLog::inverse (Analysis/src/TxnLog.cpp)
//!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item linter_format_string_match

#[cfg(test)]
#[test]
fn linter_format_string_match() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local s = ...

-- incorrect character class specifiers
string.match(s, "%q")
string.gmatch(s, "%q")
string.find(s, "%q")
string.gsub(s, "%q", "")

-- various errors
string.match(s, "%")
string.match(s, "[%1]")
string.match(s, "%0")
string.match(s, "(%d)%2")
string.match(s, "%bx")
string.match(s, "%foo")
string.match(s, '(%d))')
string.match(s, '(%d')
string.match(s, '[%d')
string.match(s, '%,')

-- self call - not detected because we don't know the type!
local _ = s:match("%q")

-- correct patterns
string.match(s, "[A-Z]+(%d)%1")
"#,
        ),
        None,
    );

    assert_eq!(14, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
        result.warnings[0].text
    );
    assert_eq!(
        "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
        result.warnings[1].text
    );
    assert_eq!(
        "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
        result.warnings[2].text
    );
    assert_eq!(
        "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
        result.warnings[3].text
    );
    assert_eq!(
        "Invalid match pattern: unfinished character class",
        result.warnings[4].text
    );
    assert_eq!(
        "Invalid match pattern: sets can not contain capture references",
        result.warnings[5].text
    );
    assert_eq!(
        "Invalid match pattern: invalid capture reference, must be 1-9",
        result.warnings[6].text
    );
    assert_eq!(
        "Invalid match pattern: invalid capture reference, must refer to a valid capture",
        result.warnings[7].text
    );
    assert_eq!(
        "Invalid match pattern: missing brace characters for balanced match",
        result.warnings[8].text
    );
    assert_eq!(
        "Invalid match pattern: missing set after a frontier pattern",
        result.warnings[9].text
    );
    assert_eq!(
        "Invalid match pattern: unexpected ) without a matching (",
        result.warnings[10].text
    );
    assert_eq!(
        "Invalid match pattern: expected ) at the end of the string to close a capture",
        result.warnings[11].text
    );
    assert_eq!(
        "Invalid match pattern: expected ] at the end of the string to close a set",
        result.warnings[12].text
    );
    assert_eq!(
        "Invalid match pattern: expected a magic character after %",
        result.warnings[13].text
    );
}
