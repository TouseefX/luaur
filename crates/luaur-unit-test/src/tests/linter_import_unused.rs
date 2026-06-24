//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:387:linter_import_unused`
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
//!   - translates_to -> rust_item linter_import_unused

#[cfg(test)]
#[test]
fn linter_import_unused() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;

    let mut fixture = Fixture::fixture_bool(false);
    let any_type = fixture.get_builtins().anyType;
    add_global_binding_builtin_definitions(
        &mut fixture.get_frontend().globals,
        "game",
        any_type,
        "@test",
    );

    let result = fixture.lint(
        &String::from(
            r#"
local Roact = require(game.Packages.Roact)
local _Roact = require(game.Packages.Roact)
"#,
        ),
        None,
    );

    assert_eq!(1, result.warnings.len(), "{:?}", result.warnings);
    assert_eq!(
        "Import 'Roact' is never used; prefix with '_' to silence",
        result.warnings[0].text.as_str()
    );
}
