//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5382:ir_builder_dominance_verification_4`
//! Source: `tests/IrBuilder.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrBuilder.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
//!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
//!   - includes -> source_file CodeGen/include/Luau/IrDump.h
//!   - includes -> source_file CodeGen/include/Luau/IrUtils.h
//!   - includes -> source_file CodeGen/include/Luau/OptimizeConstProp.h
//!   - includes -> source_file CodeGen/include/Luau/OptimizeDeadStore.h
//!   - includes -> source_file CodeGen/include/Luau/OptimizeFinalX64.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/IrBuilder.test.cpp
//! - outgoing:
//!   - calls -> method IrBuilderFixture::defineCfgTree (tests/IrBuilder.test.cpp)
//!   - type_ref -> record IdfContext (CodeGen/include/Luau/IrAnalysis.h)
//!   - calls -> function computeIteratedDominanceFrontierForDefs (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - translates_to -> rust_item ir_builder_dominance_verification_4

#[cfg(test)]
#[test]
fn ir_builder_dominance_verification_4() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::functions::compute_iterated_dominance_frontier_for_defs::compute_iterated_dominance_frontier_for_defs;
    use luaur_code_gen::records::idf_context::IdfContext;

    let mut fix = IrBuilderFixture::new();
    fix.define_cfg_tree(&vec![
        vec![1],
        vec![2, 10],
        vec![3, 7],
        vec![4],
        vec![5],
        vec![4, 6],
        vec![1],
        vec![8],
        vec![5, 9],
        vec![7],
        vec![],
    ]);

    let mut ctx = IdfContext::default();
    compute_iterated_dominance_frontier_for_defs(
        &mut ctx,
        &fix.build.function,
        &vec![0, 2, 3, 6],
        &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );
    assert_eq!(ctx.idf, vec![1, 4, 5]);
}
