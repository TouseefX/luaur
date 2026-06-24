//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1302:linter_import_only_used_in_return_type`
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
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item linter_import_only_used_in_return_type

#[cfg(test)]
#[test]
fn linter_import_only_used_in_return_type() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
        local Foo = require(script.Parent.Foo)

        function foo(): Foo.Y
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Function 'foo' is never used; prefix with '_' to silence",
        result.warnings[0].text.as_str()
    );
}
