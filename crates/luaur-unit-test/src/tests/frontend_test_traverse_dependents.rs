//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1599:frontend_test_traverse_dependents`
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
//!   - calls -> method Frontend::traverseDependents (Analysis/src/Frontend.cpp)
//!   - type_ref -> record SourceNode (Analysis/include/Luau/Frontend.h)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item frontend_test_traverse_dependents

#[cfg(test)]
#[test]
fn frontend_test_traverse_dependents() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::vec::Vec;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/A"),
        String::from("return {hello=5, world=true}"),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/B"),
        String::from(
            r#"
        return require(game:GetService('Gui').Modules.A)
    "#,
        ),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/C"),
        String::from(
            r#"
        local Modules = game:GetService('Gui').Modules
        local B = require(Modules.B)
        return {c_value = B.hello}
    "#,
        ),
    );
    fixture.base.base.file_resolver.source.insert(
        String::from("game/Gui/Modules/D"),
        String::from(
            r#"
        local Modules = game:GetService('Gui').Modules
        local C = require(Modules.C)
        return {d_value = C.c_value}
    "#,
        ),
    );

    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Gui/Modules/D"), None);

    let mut visited: Vec<String> = Vec::new();
    let visited_ptr = &mut visited as *mut Vec<String>;
    fixture.get_frontend().traverse_dependents(
        &String::from("game/Gui/Modules/B"),
        Box::new(move |node| {
            unsafe {
                (*visited_ptr).push(node.name.clone());
            }
            true
        }),
    );

    assert_eq!(
        vec![
            String::from("game/Gui/Modules/B"),
            String::from("game/Gui/Modules/C"),
            String::from("game/Gui/Modules/D"),
        ],
        visited
    );
}
