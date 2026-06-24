//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7617:ir_builder_dse_vm_exit_sync_sinking_no_inline_across_block`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function load (Config/src/LuauConfig.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_sinking_no_inline_across_block

#[cfg(test)]
#[test]
fn ir_builder_dse_vm_exit_sync_sinking_no_inline_across_block() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains;
    use luaur_code_gen::functions::optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;
    use luaur_common::FFlag;

    const TNUMBER: u8 = 3;

    let _luau_codegen_vm_exit_sync = ScopedFastFlag::new(&FFlag::LuauCodegenVmExitSync, true);

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);

        b.begin_block(block);
        let r3 = b.vm_reg(3);
        let load = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r3);
        let lhs = b.const_double(11008.0);
        let sub = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_NUM, lhs, load);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, sub);
        let r2 = b.vm_reg(2);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r2, tnumber);
        let exit = b.vm_exit(8);
        b.inst_ir_cmd_ir_op(IrCmd::CHECK_SAFE_ENV, exit);
        let r0 = b.vm_reg(0);
        let one = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);
    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0, R3\n   %0 = LOAD_DOUBLE R3\n   CHECK_SAFE_ENV bb_exit_1\n   ; exit sync: R2, {%0}\n   RETURN R0, 1i\n\nbb_exit_1:\n   %6 = SUB_NUM 11008, %0\n   STORE_TAG R2, tnumber\n   STORE_DOUBLE R2, %6\n   JUMP exit(8)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
