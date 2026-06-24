//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1119:linter_format_string_find_args`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function match (VM/src/lstrlib.cpp)
//!   - calls -> method TxnLog::inverse (Analysis/src/TxnLog.cpp)
//!   - translates_to -> rust_item linter_format_string_find_args

#[cfg(test)]
#[test]
fn linter_format_string_find_args() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local s = ...

-- incorrect character class specifier
string.find(s, "%q")

-- raw string find
string.find(s, "%q", 1, true)
string.find(s, "%q", 1, math.random() < 0.5)

-- incorrect character class specifier
string.find(s, "%q", 1, false)

-- missing arguments
string.find()
string.find("foo");
("foo"):find()
"#,
        ),
        None,
    );

    assert_eq!(2, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
        result.warnings[0].text
    );
    assert_eq!(4, result.warnings[0].location.begin.line);
    assert_eq!(
        "Invalid match pattern: invalid character class, must refer to a defined class or its inverse",
        result.warnings[1].text
    );
    assert_eq!(11, result.warnings[1].location.begin.line);
}
