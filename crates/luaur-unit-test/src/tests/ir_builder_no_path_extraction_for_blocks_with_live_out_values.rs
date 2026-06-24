//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3906:ir_builder_no_path_extraction_for_blocks_with_live_out_values`
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
//!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> function createLinearBlocks (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_no_path_extraction_for_blocks_with_live_out_values

#[cfg(test)]
#[test]
fn ir_builder_no_path_extraction_for_blocks_with_live_out_values() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::create_linear_blocks::create_linear_blocks;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block1 = b.block(IrBlockKind::Internal);
        let fallback1 = b.fallback_block(0);
        let block2 = b.block(IrBlockKind::Internal);
        let fallback2 = b.fallback_block(0);
        let block3 = b.block(IrBlockKind::Internal);
        let block4a = b.block(IrBlockKind::Internal);
        let block4b = b.block(IrBlockKind::Internal);

        b.begin_block(block1);
        let r2 = b.vm_reg(2);
        let tag1 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r2);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag1, tnumber, fallback1);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

        b.begin_block(fallback1);
        let r1 = b.vm_reg(1);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::DO_LEN, r1, r2);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, block2);

        b.begin_block(block2);
        let r2 = b.vm_reg(2);
        let tag2 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r2);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag2, tnumber, fallback2);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, block3);

        b.begin_block(fallback2);
        let r0 = b.vm_reg(0);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::DO_LEN, r0, r2);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, block3);

        b.begin_block(block3);
        let r3 = b.vm_reg(3);
        let tag3a = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r3);
        let tnil = b.const_tag(TNIL);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_EQ_TAG, tag3a, tnil, block4a, block4b);

        b.begin_block(block4a);
        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r0, tag3a);
        let r0 = b.vm_reg(0);
        let zero = b.const_int(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);

        b.begin_block(block4b);
        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r0, tag3a);
        let r0 = b.vm_reg(0);
        let zero = b.const_int(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    create_linear_blocks(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R2\n   CHECK_TAG %0, tnumber, bb_fallback_1\n   JUMP bb_2\n\nbb_fallback_1:\n   DO_LEN R1, R2\n   JUMP bb_2\n\nbb_2:\n   %5 = LOAD_TAG R2\n   CHECK_TAG %5, tnumber, bb_fallback_3\n   JUMP bb_4\n\nbb_fallback_3:\n   DO_LEN R0, R2\n   JUMP bb_4\n\nbb_4:\n   %10 = LOAD_TAG R3\n   JUMP_EQ_TAG %10, tnil, bb_5, bb_6\n\nbb_5:\n   STORE_TAG R0, %10\n   RETURN R0, 0i\n\nbb_6:\n   STORE_TAG R0, %10\n   RETURN R0, 0i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
