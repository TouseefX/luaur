//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5183:ir_builder_variadic_sequence_peeling`
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
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - translates_to -> rust_item ir_builder_variadic_sequence_peeling

#[cfg(test)]
#[test]
fn ir_builder_variadic_sequence_peeling() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);
        let a = b.block(IrBlockKind::Internal);
        let branch_b = b.block(IrBlockKind::Internal);
        let exit = b.block(IrBlockKind::Internal);

        b.begin_block(entry);
        let zero = b.const_uint(0);
        let r3 = b.vm_reg(3);
        let minus_one = b.const_int(-1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FALLBACK_GETVARARGS, zero, r3, minus_one);
        let r0 = b.vm_reg(0);
        let tag = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r0);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_EQ_TAG, tag, tnumber, a, branch_b);

        b.begin_block(a);
        let r0 = b.vm_reg(0);
        let value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r0);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r2, value);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

        b.begin_block(branch_b);
        let r1 = b.vm_reg(1);
        let value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r1);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r2, value);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

        b.begin_block(exit);
        let r2 = b.vm_reg(2);
        let minus_one = b.const_int(-1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, minus_one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1, bb_2\n; in regs: R0, R1\n; out regs: R0, R1, R3...\n   FALLBACK_GETVARARGS 0u, R3, -1i\n   %1 = LOAD_TAG R0\n   JUMP_EQ_TAG %1, tnumber, bb_1, bb_2\n\nbb_1:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R0, R3...\n; out regs: R2...\n   %3 = LOAD_TVALUE R0\n   STORE_TVALUE R2, %3\n   JUMP bb_3\n\nbb_2:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R1, R3...\n; out regs: R2...\n   %6 = LOAD_TVALUE R1\n   STORE_TVALUE R2, %6\n   JUMP bb_3\n\nbb_3:\n; predecessors: bb_1, bb_2\n; in regs: R2...\n   RETURN R2, -1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
