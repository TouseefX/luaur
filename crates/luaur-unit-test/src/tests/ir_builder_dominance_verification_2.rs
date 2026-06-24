//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5366:ir_builder_dominance_verification_2`
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
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - translates_to -> rust_item ir_builder_dominance_verification_2

#[cfg(test)]
#[test]
fn ir_builder_dominance_verification_2() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    fix.define_cfg_tree(&vec![
        vec![1, 16],
        vec![2, 3, 4],
        vec![4, 7],
        vec![9],
        vec![5],
        vec![6],
        vec![2, 8],
        vec![8],
        vec![7, 15],
        vec![10, 11],
        vec![12],
        vec![12],
        vec![13],
        vec![3, 14, 15],
        vec![12],
        vec![16],
        vec![],
    ]);

    assert_eq!(
        fix.build.function.cfg.idoms,
        vec![!0u32, 0, 1, 1, 1, 4, 5, 1, 1, 3, 9, 9, 9, 12, 13, 1, 0]
    );
}
