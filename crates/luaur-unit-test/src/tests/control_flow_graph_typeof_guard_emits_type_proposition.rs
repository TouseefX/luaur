//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ControlFlowGraph.test.cpp:381:control_flow_graph_typeof_guard_emits_type_proposition`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Block (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function checkRefine (tests/ControlFlowGraph.test.cpp)
//!   - calls -> function requireInst (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Refine (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item control_flow_graph_typeof_guard_emits_type_proposition

#[cfg(test)]
#[test]
fn control_flow_graph_typeof_guard_emits_type_proposition() {
    use crate::functions::check_refine::check_refine;
    use crate::functions::require_inst::require_inst;
    use crate::records::cfg_fixture::CfgFixture;
    use luaur_analysis::records::refine::Refine;

    let mut fixture = CfgFixture::default();
    let cfg = fixture.build(
        r#"
        local x = nil
        if typeof(x) == "string" then
            local y = x
        end
    "#,
    );

    let cfg = unsafe { &*cfg };
    assert_eq!(4, cfg.blocks.len());

    let then_blk = cfg.blocks[1];
    let else_blk = cfg.blocks[2];

    check_refine(
        require_inst::<Refine>(then_blk, 0),
        "x-1",
        "x-0",
        true,
        Some("string"),
        true,
    );
    check_refine(
        require_inst::<Refine>(else_blk, 0),
        "x-2",
        "x-0",
        false,
        Some("string"),
        true,
    );
}
