//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:241:control_flow_graph_basic_join`
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
//!   - calls -> function merge (tests/LValue.test.cpp)
//!   - type_ref -> enum BlockKind (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Entry (Ast/include/Luau/Lexer.h)
//!   - calls -> function checkSuccessors (tests/ControlFlowGraph.test.cpp)
//!   - calls -> function checkPredecessors (tests/ControlFlowGraph.test.cpp)
//!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Assign (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Join (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function checkJoin (tests/ControlFlowGraph.test.cpp)
//!   - translates_to -> rust_item control_flow_graph_basic_join

#[cfg(test)]
#[test]
fn control_flow_graph_basic_join() {
    use crate::functions::check_join::check_join;
    use crate::functions::check_predecessors::check_predecessors;
    use crate::functions::check_successors::check_successors;
    use crate::functions::require_inst::require_inst;
    use crate::records::cfg_fixture::CfgFixture;
    use luaur_analysis::enums::block_kind::BlockKind;
    use luaur_analysis::records::assign::Assign;
    use luaur_analysis::records::declare::Declare;
    use luaur_analysis::records::join::Join;

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
        r#"
        local t = 8
        if true then
            t = 9
        else
            t = "hello"
        end
        local y = t
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let entry = cfg.blocks[0];
    let then_blk = cfg.blocks[1];
    let else_blk = cfg.blocks[2];
    let merge = cfg.blocks[3];

    assert_eq!(BlockKind::Entry, unsafe { (*entry).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*then_blk).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*else_blk).kind });
    assert_eq!(BlockKind::Linear, unsafe { (*merge).kind });

    check_successors(cfg, entry, &[1, 2]);
    check_successors(cfg, then_blk, &[3]);
    check_successors(cfg, else_blk, &[3]);
    check_predecessors(cfg, merge, &[1, 2]);

    assert_eq!("t-0", unsafe {
        (*(*require_inst::<Declare>(entry, 0)).def).versioned_name()
    });
    assert_eq!("t-1", unsafe {
        (*(*require_inst::<Assign>(then_blk, 0)).def).versioned_name()
    });
    assert_eq!("t-2", unsafe {
        (*(*require_inst::<Assign>(else_blk, 0)).def).versioned_name()
    });

    let phi = require_inst::<Join>(merge, 0);
    check_join(phi, "t-3", &["t-1", "t-2"]);

    let decl_y = require_inst::<Declare>(merge, 1);
    assert_eq!("y-0", unsafe { (*(*decl_y).def).versioned_name() });
}
