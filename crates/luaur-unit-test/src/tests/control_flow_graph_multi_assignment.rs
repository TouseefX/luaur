//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:220:control_flow_graph_multi_assignment`
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
//!   - type_ref -> record Assign (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item control_flow_graph_multi_assignment

#[cfg(test)]
#[test]
fn control_flow_graph_multi_assignment() {
    use crate::functions::require_inst::require_inst;
    use crate::records::cfg_fixture::CfgFixture;
    use luaur_analysis::records::assign::Assign;
    use luaur_analysis::records::declare::Declare;
    use luaur_ast::records::position::Position;

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
        r#"
        local a, b = 1, 2
        a, b = b, a
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(1, cfg.blocks.len());

    let entry = cfg.blocks[0];

    assert_eq!("a-0", unsafe {
        (*(*require_inst::<Declare>(entry, 0)).def).versioned_name()
    });
    assert_eq!("b-0", unsafe {
        (*(*require_inst::<Declare>(entry, 1)).def).versioned_name()
    });
    assert_eq!("a-1", unsafe {
        (*(*require_inst::<Assign>(entry, 2)).def).versioned_name()
    });
    assert_eq!("b-1", unsafe {
        (*(*require_inst::<Assign>(entry, 3)).def).versioned_name()
    });

    assert_eq!("b-0", unsafe {
        (*fixture.get_definition_at_pos(cfg, Position::new(2, 15))).versioned_name()
    });
    assert_eq!("a-0", unsafe {
        (*fixture.get_definition_at_pos(cfg, Position::new(2, 18))).versioned_name()
    });
}
