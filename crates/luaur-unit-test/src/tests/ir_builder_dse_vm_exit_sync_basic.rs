//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7259:ir_builder_dse_vm_exit_sync_basic`
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
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_dse_vm_exit_sync_basic

#[cfg(test)]
#[test]
fn ir_builder_dse_vm_exit_sync_basic() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains;
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
        let r1 = b.vm_reg(1);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, tnumber);
        let r1 = b.vm_reg(1);
        let two = b.const_double(2.0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r1, two);
        let r0 = b.vm_reg(0);
        let tag = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r0);
        let tnumber = b.const_tag(TNUMBER);
        let exit = b.vm_exit(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag, tnumber, exit);
        let r0 = b.vm_reg(0);
        let one = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   %2 = LOAD_TAG R0\n   CHECK_TAG %2, tnumber, bb_exit_1\n   ; exit sync: R1, {}\n   RETURN R0, 1i\n\nbb_exit_1:\n   STORE_TAG R1, tnumber\n   STORE_DOUBLE R1, 2\n   JUMP exit(1)\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
