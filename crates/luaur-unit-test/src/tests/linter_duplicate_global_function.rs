//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1404:linter_duplicate_global_function`
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
//!   - type_ref -> record LintWarning (Config/include/Luau/LinterConfig.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item linter_duplicate_global_function

#[cfg(test)]
#[test]
fn linter_duplicate_global_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_config::enums::code::Code;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
        function x() end

        function x() end

        return x
    "#,
        ),
        None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);

    let warning = &result.warnings[0];
    assert_eq!(Code::Code_DuplicateFunction, warning.code);
    assert_eq!(
        "Duplicate function definition: 'x' also defined on line 2",
        warning.text.as_str()
    );
}
