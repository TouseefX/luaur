//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2124:linter_table_operations`
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
//!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
//!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item linter_table_operations

#[cfg(test)]
#[test]
fn linter_table_operations() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.lint(
        &String::from(
            r#"
local t = {}
local tt = {}

table.insert(t, #t, 42)
table.insert(t, (#t), 42) -- silenced

table.insert(t, #t + 1, 42)
table.insert(t, #tt + 1, 42) -- different table, ok

table.insert(t, 0, 42)

table.remove(t, 0)

table.remove(t, #t-1)

table.insert(t, string.find("hello", "h"))

table.move(t, 0, #t, 1, tt)
table.move(t, 1, #t, 0, tt)

table.create(42, {})
table.create(42, {} :: {})
"#,
        ),
        None,
    );

    assert_eq!(10, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        concat!(
            "table.insert will insert the value before the last element, which is likely a bug; consider removing the ",
            "second argument or wrap it in parentheses to silence"
        ),
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "table.insert will append the value to the table; consider removing the second argument for efficiency",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "table.insert uses index 0 but arrays are 1-based; did you mean 1 instead?",
        result.warnings[2].text.as_str()
    );
    assert_eq!(
        "table.remove uses index 0 but arrays are 1-based; did you mean 1 instead?",
        result.warnings[3].text.as_str()
    );
    assert_eq!(
        concat!(
            "table.remove will remove the value before the last element, which is likely a bug; consider removing the ",
            "second argument or wrap it in parentheses to silence"
        ),
        result.warnings[4].text.as_str()
    );
    assert_eq!(
        "table.insert may change behavior if the call returns more than one result; consider adding parentheses around second argument",
        result.warnings[5].text.as_str()
    );
    assert_eq!(
        "table.move uses index 0 but arrays are 1-based; did you mean 1 instead?",
        result.warnings[6].text.as_str()
    );
    assert_eq!(
        "table.move uses index 0 but arrays are 1-based; did you mean 1 instead?",
        result.warnings[7].text.as_str()
    );
    assert_eq!(
        "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead",
        result.warnings[8].text.as_str()
    );
    assert_eq!(
        "table.create with a table literal will reuse the same object for all elements; consider using a for loop instead",
        result.warnings[9].text.as_str()
    );
}
