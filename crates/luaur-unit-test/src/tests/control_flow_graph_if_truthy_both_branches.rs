//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:328:control_flow_graph_if_truthy_both_branches`
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
//!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
//!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Declare (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method SymDef::versionedName (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Join (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function checkJoin (tests/ControlFlowGraph.test.cpp)
//!   - translates_to -> rust_item control_flow_graph_if_truthy_both_branches

#[cfg(test)]
#[test]
fn control_flow_graph_if_truthy_both_branches() {
    use crate::functions::check_join::check_join;
    use crate::functions::check_refine::check_refine;
    use crate::functions::require_inst::require_inst;
    use crate::records::cfg_fixture::CfgFixture;
    use luaur_analysis::records::declare::Declare;
    use luaur_analysis::records::join::Join;
    use luaur_analysis::records::refine::Refine;

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
        r#"
        local x = nil
        if x then
            local y = x
        else
            local z = x
        end

        local y = x
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let then_blk = cfg.blocks[1];
    let else_blk = cfg.blocks[2];
    let merge = cfg.blocks[3];

    check_refine(
        require_inst::<Refine>(then_blk, 0),
        "x-1",
        "x-0",
        true,
        None,
        false,
    );
    assert_eq!("y-0", unsafe {
        (*(*require_inst::<Declare>(then_blk, 1)).def).versioned_name()
    });

    check_refine(
        require_inst::<Refine>(else_blk, 0),
        "x-2",
        "x-0",
        false,
        None,
        false,
    );
    assert_eq!("z-0", unsafe {
        (*(*require_inst::<Declare>(else_blk, 1)).def).versioned_name()
    });

    let phi = require_inst::<Join>(merge, 0);
    check_join(phi, "x-3", &["x-1", "x-2"]);
    assert_eq!("y-0", unsafe {
        (*(*require_inst::<Declare>(merge, 1)).def).versioned_name()
    });
}
