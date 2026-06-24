//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1210:linter_table_literal`
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
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item linter_table_literal

#[cfg(test)]
#[test]
fn linter_table_literal() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"-- line 1
_ = {
    first = 1,
    second = 2,
    first = 3,
}

_ = {
    first = 1,
    ["first"] = 2,
}

_ = {
    1, 2, 3,
    [1] = 42
}

_ = {
    [3] = 42,
    1, 2, 3,
}

local _: {
    first: number,
    second: string,
    first: boolean
}

_ = {
    1, 2, 3,
    [0] = 42,
    [4] = 42,
}

_ = {
    [1] = 1,
    [2] = 2,
    [1] = 3,
}

function _foo(): { first: number, second: string, first: boolean }
end
"#,
        ),
        None,
    );

    assert_eq!(7, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Table field 'first' is a duplicate; previously defined at line 3",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Table field 'first' is a duplicate; previously defined at line 9",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "Table index 1 is a duplicate; previously defined as a list entry",
        result.warnings[2].text.as_str()
    );
    assert_eq!(
        "Table index 3 is a duplicate; previously defined as a list entry",
        result.warnings[3].text.as_str()
    );
    assert_eq!(
        "Table type field 'first' is a duplicate; previously defined at line 24",
        result.warnings[4].text.as_str()
    );
    assert_eq!(
        "Table index 1 is a duplicate; previously defined at line 36",
        result.warnings[5].text.as_str()
    );
    assert_eq!(
        "Table type field 'first' is a duplicate; previously defined at line 41",
        result.warnings[6].text.as_str()
    );
}
