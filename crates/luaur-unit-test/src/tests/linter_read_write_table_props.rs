//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1266:linter_read_write_table_props`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - translates_to -> rust_item linter_read_write_table_props

#[cfg(test)]
#[test]
fn linter_read_write_table_props() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_OLD_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"-- line 1
        type A = {x: number}
        type B = {read x: number, write x: number}
        type C = {x: number, read x: number} -- line 4
        type D = {x: number, write x: number}
        type E = {read x: number, x: boolean}
        type F = {read x: number, read x: number}
        type G = {write x: number, x: boolean}
        type H = {write x: number, write x: boolean}
    "#,
        ),
        None,
    );

    assert_eq!(6, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Table type field 'x' is already read-write; previously defined at line 4",
        result.warnings[0].text.as_str()
    );
    assert_eq!(
        "Table type field 'x' is already read-write; previously defined at line 5",
        result.warnings[1].text.as_str()
    );
    assert_eq!(
        "Table type field 'x' already has a read type defined at line 6",
        result.warnings[2].text.as_str()
    );
    assert_eq!(
        "Table type field 'x' is a duplicate; previously defined at line 7",
        result.warnings[3].text.as_str()
    );
    assert_eq!(
        "Table type field 'x' already has a write type defined at line 8",
        result.warnings[4].text.as_str()
    );
    assert_eq!(
        "Table type field 'x' is a duplicate; previously defined at line 9",
        result.warnings[5].text.as_str()
    );
}
