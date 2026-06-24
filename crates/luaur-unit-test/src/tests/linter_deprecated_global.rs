//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:48:linter_deprecated_global`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lint (tests/Fixture.cpp)
//!   - translates_to -> rust_item linter_deprecated_global

#[cfg(test)]
#[test]
fn linter_deprecated_global() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::add_global_binding_builtin_definitions_alt_b::add_global_binding_builtin_definitions_alt_b;
    use luaur_analysis::records::binding::Binding;
    use luaur_ast::records::location::Location;

    let mut fixture = Fixture::fixture_bool(false);
    let any_type = fixture.get_builtins().anyType;
    add_global_binding_builtin_definitions_alt_b(
        &mut fixture.get_frontend().globals,
        "Wait",
        Binding {
            type_id: any_type,
            location: Location::default(),
            deprecated: true,
            deprecated_suggestion: String::from("wait"),
            documentation_symbol: Some(String::from("@test/global/Wait")),
        },
    );

    let result = fixture.lint(&String::from("Wait(5)"), None);

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Global 'Wait' is deprecated, use 'wait' instead",
        result.warnings[0].text.as_str()
    );
}
