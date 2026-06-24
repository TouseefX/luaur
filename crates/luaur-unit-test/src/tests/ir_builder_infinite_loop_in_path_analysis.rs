//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3985:ir_builder_infinite_loop_in_path_analysis`
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
//!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> function createLinearBlocks (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_infinite_loop_in_path_analysis

#[cfg(test)]
#[test]
fn ir_builder_infinite_loop_in_path_analysis() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::create_linear_blocks::create_linear_blocks;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block1 = b.block(IrBlockKind::Internal);
        let block2 = b.block(IrBlockKind::Internal);

        b.begin_block(block1);
        let r0 = b.vm_reg(0);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r0, tnumber);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

        b.begin_block(block2);
        let r1 = b.vm_reg(1);
        let tboolean = b.const_tag(TBOOLEAN);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, tboolean);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    create_linear_blocks(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   JUMP bb_1\n\nbb_1:\n   STORE_TAG R1, tboolean\n   JUMP bb_1\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
