//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5871:ir_builder_indirect_float_load_extraction_must_respect_version`
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
//!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_indirect_float_load_extraction_must_respect_version

#[cfg(test)]
#[test]
fn ir_builder_indirect_float_load_extraction_must_respect_version() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNUMBER: u8 = 3;
    const TVECTOR: u8 = 5;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);

        b.begin_block(entry);

        let r4 = b.vm_reg(4);
        let tag4 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r4);
        let tvector = b.const_tag(TVECTOR);
        let exit = b.vm_exit(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag4, tvector, exit);
        let r5 = b.vm_reg(5);
        let tag5 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r5);
        let tvector = b.const_tag(TVECTOR);
        let exit = b.vm_exit(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag5, tvector, exit);

        let r4 = b.vm_reg(4);
        let zero = b.const_int(0);
        let x1_float = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, r4, zero);
        let x1 = b.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, x1_float);
        let r5 = b.vm_reg(5);
        let zero = b.const_int(0);
        let x2_float = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, r5, zero);
        let x2 = b.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, x2_float);

        let min = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MIN_NUM, x1, x2);
        let x_min = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, min);
        let r7 = b.vm_reg(7);
        let zero_a = b.const_double(0.0);
        let zero_b = b.const_double(0.0);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::STORE_VECTOR, r7, x_min, zero_a, zero_b);
        let r7 = b.vm_reg(7);
        let tvector = b.const_tag(TVECTOR);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r7, tvector);

        let r7 = b.vm_reg(7);
        let zero = b.const_int(0);
        let tvector = b.const_tag(TVECTOR);
        let x_min_vec = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LOAD_TVALUE, r7, zero, tvector);
        let r6 = b.vm_reg(6);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r6, x_min_vec);

        let max = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MAX_NUM, x1, x2);
        let x_max = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_FLOAT, max);
        let r7 = b.vm_reg(7);
        let zero_a = b.const_double(0.0);
        let zero_b = b.const_double(0.0);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::STORE_VECTOR, r7, x_max, zero_a, zero_b);
        let r7 = b.vm_reg(7);
        let tvector = b.const_tag(TVECTOR);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r7, tvector);

        let r7 = b.vm_reg(7);
        let zero = b.const_int(0);
        let tvector = b.const_tag(TVECTOR);
        let x_max_vec = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LOAD_TVALUE, r7, zero, tvector);
        let r5 = b.vm_reg(5);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r5, x_max_vec);

        let r4 = b.vm_reg(4);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r4, x_min_vec);

        let r4 = b.vm_reg(4);
        let zero = b.const_int(0);
        let x_min_copy_float = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_FLOAT, r4, zero);
        let x_min_copy = b.inst_ir_cmd_ir_op(IrCmd::FLOAT_TO_NUM, x_min_copy_float);
        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r0, x_min_copy);
        let r0 = b.vm_reg(0);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r0, tnumber);
        let r0 = b.vm_reg(0);
        let one = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R4, R5\n   %0 = LOAD_TAG R4\n   CHECK_TAG %0, tvector, exit(1)\n   %2 = LOAD_TAG R5\n   CHECK_TAG %2, tvector, exit(1)\n   %4 = LOAD_FLOAT R4, 0i\n   %5 = FLOAT_TO_NUM %4\n   %6 = LOAD_FLOAT R5, 0i\n   %7 = FLOAT_TO_NUM %6\n   %8 = MIN_NUM %5, %7\n   %9 = NUM_TO_FLOAT %8\n   STORE_VECTOR R7, %9, 0, 0\n   STORE_TAG R7, tvector\n   %12 = LOAD_TVALUE R7, 0i, tvector\n   STORE_TVALUE R6, %12\n   %14 = MAX_NUM %5, %7\n   %15 = NUM_TO_FLOAT %14\n   STORE_VECTOR R7, %15, 0, 0\n   %18 = LOAD_TVALUE R7, 0i, tvector\n   STORE_TVALUE R5, %18\n   STORE_TVALUE R4, %12\n   %21 = EXTRACT_VEC %12, 0i\n   %22 = FLOAT_TO_NUM %21\n   STORE_DOUBLE R0, %22\n   STORE_TAG R0, tnumber\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
