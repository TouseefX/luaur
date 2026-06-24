//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5374:ir_builder_dominance_verification_3`
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
//!   - translates_to -> rust_item ir_builder_dominance_verification_3

#[cfg(test)]
#[test]
fn ir_builder_dominance_verification_3() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;

    let mut fix = IrBuilderFixture::new();
    fix.define_cfg_tree(&vec![
        vec![1, 2],
        vec![3],
        vec![3, 4],
        vec![5],
        vec![5, 6],
        vec![7],
        vec![7],
        vec![],
    ]);

    assert_eq!(
        fix.build.function.cfg.idoms,
        vec![!0u32, 0, 0, 0, 2, 0, 4, 0]
    );
}
