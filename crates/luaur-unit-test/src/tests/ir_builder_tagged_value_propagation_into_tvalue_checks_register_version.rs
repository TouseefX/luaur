//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5819:ir_builder_tagged_value_propagation_into_tvalue_checks_register_version`
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
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_tagged_value_propagation_into_tvalue_checks_register_version

#[cfg(test)]
#[test]
fn ir_builder_tagged_value_propagation_into_tvalue_checks_register_version() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);

        b.begin_block(entry);
        let r0 = b.vm_reg(0);
        let a1 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r0);
        let r1 = b.vm_reg(1);
        let b1 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r1);
        let sum1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, a1, b1);

        let r7 = b.vm_reg(7);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r7, sum1);
        let r7 = b.vm_reg(7);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r7, tnumber);

        let r2 = b.vm_reg(2);
        let a2 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r2);
        let r3 = b.vm_reg(3);
        let b2 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r3);
        let sum2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, a2, b2);

        let r8 = b.vm_reg(8);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r8, sum2);
        let r8 = b.vm_reg(8);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r8, tnumber);

        let r7 = b.vm_reg(7);
        let zero = b.const_int(0);
        let tnumber = b.const_tag(TNUMBER);
        let old7 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LOAD_TVALUE, r7, zero, tnumber);
        let r8 = b.vm_reg(8);
        let zero = b.const_int(0);
        let tnumber = b.const_tag(TNUMBER);
        let old8 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::LOAD_TVALUE, r8, zero, tnumber);

        let r8 = b.vm_reg(8);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r8, old7);
        let r9 = b.vm_reg(9);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r9, old8);

        let r8 = b.vm_reg(8);
        let two = b.const_int(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r8, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R1, R2, R3\n   %0 = LOAD_DOUBLE R0\n   %1 = LOAD_DOUBLE R1\n   %2 = ADD_NUM %0, %1\n   STORE_DOUBLE R7, %2\n   STORE_TAG R7, tnumber\n   %5 = LOAD_DOUBLE R2\n   %6 = LOAD_DOUBLE R3\n   %7 = ADD_NUM %5, %6\n   STORE_DOUBLE R8, %7\n   STORE_TAG R8, tnumber\n   %11 = LOAD_TVALUE R8, 0i, tnumber\n   STORE_SPLIT_TVALUE R8, tnumber, %2\n   STORE_TVALUE R9, %11\n   RETURN R8, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
