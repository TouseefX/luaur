//! Ported from `tests/Linter.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Linter.test.cpp:1337:linter_use_all_parent_scopes_for_globals`
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
//!   - calls -> method Frontend::addEnvironment (Analysis/src/Frontend.cpp)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record LintResult (Analysis/include/Luau/Linter.h)
//!   - calls -> method Fixture::lintModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item linter_use_all_parent_scopes_for_globals

#[cfg(test)]
#[test]
fn linter_use_all_parent_scopes_for_globals() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::frontend::Frontend;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let test_scope = fixture.get_frontend().add_environment(String::from("Test"));
    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    unsafe {
        unfreeze((*frontend_ptr).globals.global_types_mut());
        let result = (*frontend_ptr).load_definition_file(
            &mut (*frontend_ptr).globals,
            test_scope,
            r#"
        declare Foo: number
    "#,
            String::from("@test"),
            false,
            false,
        );
        assert!(result.success, "{:?}", result);
        freeze((*frontend_ptr).globals.global_types_mut());
    }

    fixture
        .base
        .file_resolver
        .environments
        .insert(String::from("A"), String::from("Test"));
    fixture.base.file_resolver.source.insert(
        String::from("A"),
        String::from(
            r#"
        local _foo: Foo = 123
        -- os.clock comes from the global scope, the parent of this module's environment
        local _bar: typeof(os.clock) = os.clock
    "#,
        ),
    );

    let result = fixture.base.lint_module(&String::from("A"), None);

    assert_eq!(0, result.warnings.len(), "{:?}", result.warnings);
}
