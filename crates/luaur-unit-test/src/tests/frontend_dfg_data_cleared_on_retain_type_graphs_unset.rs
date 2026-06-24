//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1572:frontend_dfg_data_cleared_on_retain_type_graphs_unset`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method FrontendFixture::getFrontend (tests/Frontend.test.cpp)
//!   - calls -> method TestFileResolver::getModule (tests/Fixture.cpp)
//!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
//!   - calls -> method Frontend::markDirty (Analysis/src/Frontend.cpp)
//!   - translates_to -> rust_item frontend_dfg_data_cleared_on_retain_type_graphs_unset

#[cfg(test)]
#[test]
fn frontend_dfg_data_cleared_on_retain_type_graphs_unset() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.file_resolver.source.insert(
        String::from("game/A"),
        String::from(
            r#"
local a = 1
local b = 2
local c = 3
return {x = a, y = b, z = c}
"#,
        ),
    );

    fixture.get_frontend().options.retain_full_type_graphs = true;
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    assert!(!module.def_arena.allocator.empty());
    assert!(!module.key_arena.empty());

    fixture.get_frontend().options.retain_full_type_graphs = false;
    fixture
        .get_frontend()
        .mark_dirty(&String::from("game/A"), None);
    fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/A"), None);

    let module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/A"));
    assert!(module.def_arena.allocator.empty());
    assert!(module.key_arena.empty());
}
