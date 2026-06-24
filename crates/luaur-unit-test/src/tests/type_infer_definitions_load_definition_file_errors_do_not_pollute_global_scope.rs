//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:81:type_infer_definitions_load_definition_file_errors_do_not_pollute_global_scope`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function tryGetGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_definitions_load_definition_file_errors_do_not_pollute_global_scope

#[cfg(test)]
#[test]
fn type_infer_definitions_load_definition_file_errors_do_not_pollute_global_scope() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::try_get_global_binding::try_get_global_binding;
    use luaur_analysis::records::frontend::Frontend;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let parse_fail_result = unsafe {
        let target_scope = (*frontend_ptr).globals.global_scope();
        (*frontend_ptr).load_definition_file(
            &mut (*frontend_ptr).globals,
            target_scope,
            r#"
        declare foo
    "#,
            String::from("@test"),
            false,
            false,
        )
    };

    assert!(!parse_fail_result.success);
    let foo_ty = unsafe { try_get_global_binding(&mut (*frontend_ptr).globals, "foo") };
    assert!(foo_ty.is_none());

    let check_fail_result = unsafe {
        let target_scope = (*frontend_ptr).globals.global_scope();
        (*frontend_ptr).load_definition_file(
            &mut (*frontend_ptr).globals,
            target_scope,
            r#"
        local foo: string = 123
        declare bar: typeof(foo)
    "#,
            String::from("@test"),
            false,
            false,
        )
    };

    assert!(!check_fail_result.success);
    let bar_ty = unsafe { try_get_global_binding(&mut (*frontend_ptr).globals, "bar") };
    assert!(bar_ty.is_none());
}
