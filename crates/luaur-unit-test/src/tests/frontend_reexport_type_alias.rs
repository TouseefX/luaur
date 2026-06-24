//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1231:frontend_reexport_type_alias`
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
//!   - type_ref -> record Module (Analysis/include/Luau/Module.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - translates_to -> rust_item frontend_reexport_type_alias

#[cfg(test)]
#[test]
fn frontend_reexport_type_alias() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();

    fixture.base.file_resolver.source.insert(
        String::from("Module/A"),
        String::from(
            r#"
        type KeyOfTestEvents = "test-file-start" | "test-file-success" | "test-file-failure" | "test-case-result"
        type MyAny = any

        export type TestFileEvent<T = KeyOfTestEvents> = (
            eventName: T,
            args: any --[[ ROBLOX TODO: Unhandled node for type: TSIndexedAccessType ]] --[[ TestEvents[T] ]]
        ) -> MyAny

        return {}
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("Module/B"),
        String::from(
            r#"
        --!strict
        local A = require(script.Parent.A)

        export type TestFileEvent = A.TestFileEvent
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Module/B"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
