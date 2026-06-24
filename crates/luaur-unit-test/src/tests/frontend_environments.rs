//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:984:frontend_environments`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method Frontend::addEnvironment (Analysis/src/Frontend.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item frontend_environments

#[cfg(test)]
#[test]
fn frontend_environments() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::frontend::Frontend;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    let test_scope = fixture.get_frontend().add_environment(String::from("test"));

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    unsafe {
        unfreeze((*frontend_ptr).globals.global_types_mut());
        let result = (*frontend_ptr).load_definition_file(
            &mut (*frontend_ptr).globals,
            test_scope,
            r#"
        export type Foo = number | string
    "#,
            String::from("@test"),
            false,
            false,
        );
        assert!(result.success, "{:?}", result);
        freeze((*frontend_ptr).globals.global_types_mut());
    }

    fixture.base.base.file_resolver.source.insert(
        String::from("A"),
        String::from(
            r#"
        --!nonstrict
        local foo: Foo = 1
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("B"),
        String::from(
            r#"
        --!nonstrict
        local foo: Foo = 1
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("C"),
        String::from(
            r#"
        --!strict
        local foo: Foo = 1
    "#,
        ),
    );

    fixture
        .base
        .base
        .file_resolver
        .environments
        .insert(String::from("A"), String::from("test"));

    let result_a = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("A"), None);
    assert_eq!(0, result_a.errors.len(), "{:?}", result_a.errors);

    let result_b = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("B"), None);
    assert_eq!(1, result_b.errors.len(), "{:?}", result_b.errors);

    let result_c = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("C"), None);
    assert_eq!(1, result_c.errors.len(), "{:?}", result_c.errors);
}
