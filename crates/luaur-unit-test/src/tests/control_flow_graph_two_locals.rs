//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:172:control_flow_graph_two_locals`
//! Source: `tests/ControlFlowGraph.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ControlFlowGraph.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/ControlFlowGraph.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/DumpCFG.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/ControlFlowGraph.test.cpp
//! - outgoing:
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item control_flow_graph_two_locals

#[cfg(test)]
#[test]
fn control_flow_graph_two_locals() {
    use crate::functions::require_inst::require_inst;
    use crate::records::cfg_fixture::CfgFixture;
    use luaur_analysis::records::declare::Declare;

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
        r#"
        local x = 4
        local y = 5
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(1, cfg.blocks.len());

    let entry = cfg.blocks[0];

    let decl_x = require_inst::<Declare>(entry, 0);
    assert_eq!("x-0", unsafe { (*(*decl_x).def).versioned_name() });

    let decl_y = require_inst::<Declare>(entry, 1);
    assert_eq!("y-0", unsafe { (*(*decl_y).def).versioned_name() });
}
