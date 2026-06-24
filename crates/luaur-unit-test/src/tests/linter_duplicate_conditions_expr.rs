//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:2277:linter_duplicate_conditions_expr`
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
//!   - translates_to -> rust_item linter_duplicate_conditions_expr

#[cfg(test)]
#[test]
fn linter_duplicate_conditions_expr() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.lint(
        &String::from(
            r#"
local correct, opaque = ...

if correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", `string {opaque}`)}) then
elseif correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", `string {opaque}`)}) then
elseif correct({a = 1, b = 2 * (-2), c = opaque.path['with']("calls", false)}) then
end
"#,
        ),
        None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Condition has already been checked on line 4",
        result.warnings[0].text.as_str()
    );
    assert_eq!(5, result.warnings[0].location.begin.line + 1);
}
