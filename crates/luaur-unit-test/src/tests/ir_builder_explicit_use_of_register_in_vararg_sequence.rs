//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5058:ir_builder_explicit_use_of_register_in_vararg_sequence`
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
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - translates_to -> rust_item ir_builder_explicit_use_of_register_in_vararg_sequence

#[cfg(test)]
#[test]
fn ir_builder_explicit_use_of_register_in_vararg_sequence() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);
        let exit = b.block(IrBlockKind::Internal);

        b.begin_block(entry);
        let zero = b.const_uint(0);
        let r1 = b.vm_reg(1);
        let minus_one = b.const_int(-1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FALLBACK_GETVARARGS, zero, r1, minus_one);
        let zero = b.const_uint(0);
        let r0 = b.vm_reg(0);
        let r1 = b.vm_reg(1);
        let r2 = b.vm_reg(2);
        let undef = b.undef();
        let minus_one_params = b.const_int(-1);
        let minus_one_results = b.const_int(-1);
        let results = b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::INVOKE_FASTCALL,
            zero,
            r0,
            r1,
            r2,
            undef,
            minus_one_params,
            minus_one_results,
        );
        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::ADJUST_STACK_TO_REG, r0, results);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

        b.begin_block(exit);
        let r0 = b.vm_reg(0);
        let minus_one = b.const_int(-1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, minus_one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1\n; out regs: R0...\n   FALLBACK_GETVARARGS 0u, R1, -1i\n   %1 = INVOKE_FASTCALL 0u, R0, R1, R2, undef, -1i, -1i\n   ADJUST_STACK_TO_REG R0, %1\n   JUMP bb_1\n\nbb_1:\n; predecessors: bb_0\n; in regs: R0...\n   RETURN R0, -1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
