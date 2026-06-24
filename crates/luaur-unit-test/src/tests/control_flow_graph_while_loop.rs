//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:280:control_flow_graph_while_loop`
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
//!   - type_ref -> enum BlockKind (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
//!   - calls -> function checkSuccessors (tests/ControlFlowGraph.test.cpp)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function checkPredecessors (tests/ControlFlowGraph.test.cpp)
//!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Join (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function checkJoin (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Assign (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item control_flow_graph_while_loop

#[cfg(test)]
#[test]
fn control_flow_graph_while_loop() {
    use crate::functions::check_join::check_join;
    use crate::functions::check_predecessors::check_predecessors;
    use crate::functions::check_refine::check_refine;
    use crate::functions::check_successors::check_successors;
    use crate::functions::require_inst::require_inst;
    use crate::records::cfg_fixture::CfgFixture;
    use luaur_analysis::enums::block_kind::BlockKind;
    use luaur_analysis::records::assign::Assign;
    use luaur_analysis::records::declare::Declare;
    use luaur_analysis::records::join::Join;
    use luaur_analysis::records::refine::Refine;

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
        r#"
        local x = nil
        while not x do
            x = 5
        end
        local y = x
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let entry = cfg.blocks[0];
    let header = cfg.blocks[1];
    let body = cfg.blocks[2];
    let exit = cfg.blocks[3];

    assert_eq!(BlockKind::Entry, unsafe { (*entry).kind });
    assert_eq!(BlockKind::Condition, unsafe { (*header).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*body).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*exit).kind });

    check_successors(cfg, entry, &[1]);
    check_successors(cfg, header, &[2, 3]);
    check_successors(cfg, body, &[1]);
    check_predecessors(cfg, header, &[0, 2]);

    assert_eq!("x-0", unsafe {
        (*(*require_inst::<Declare>(entry, 0)).def).versioned_name()
    });

    let phi = require_inst::<Join>(header, 0);
    check_join(phi, "x-1", &["x-0", "x-3"]);

    let body_refine = require_inst::<Refine>(body, 0);
    check_refine(body_refine, "x-2", "x-1", false, None, false);
    assert_eq!("x-3", unsafe {
        (*(*require_inst::<Assign>(body, 1)).def).versioned_name()
    });

    let exit_refine = require_inst::<Refine>(exit, 0);
    check_refine(exit_refine, "x-4", "x-1", true, None, false);
    assert_eq!("y-0", unsafe {
        (*(*require_inst::<Declare>(exit, 1)).def).versioned_name()
    });
}
