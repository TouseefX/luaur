//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1539:frontend_check_module_references_allocator`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item frontend_check_module_references_allocator

#[cfg(test)]
#[test]
fn frontend_check_module_references_allocator() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use alloc::sync::Arc;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/workspace/MyScript"),
        String::from(
            r#"
        print("Hello World")
    "#,
        ),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(
            &String::from("game/workspace/MyScript"),
            None,
        );

    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/workspace/MyScript"));
    let source = fixture
        .get_frontend()
        .get_source_module(&String::from("game/workspace/MyScript"));
    assert!(!source.is_null());

    let source = unsafe { &*source };
    assert_eq!(
        Arc::as_ptr(
            module
                .allocator
                .as_ref()
                .expect("expected module allocator")
        ),
        Arc::as_ptr(&source.allocator)
    );
    assert_eq!(
        Arc::as_ptr(module.names.as_ref().expect("expected module names")),
        Arc::as_ptr(&source.names)
    );
}
