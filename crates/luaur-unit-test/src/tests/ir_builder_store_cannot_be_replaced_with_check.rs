//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6445:ir_builder_store_cannot_be_replaced_with_check`
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
//!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - translates_to -> rust_item ir_builder_store_cannot_be_replaced_with_check

#[cfg(test)]
#[test]
fn ir_builder_store_cannot_be_replaced_with_check() {
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

    const TNIL: u8 = 0;
    const TTABLE: u8 = 7;

    let _debug_luau_aborting_checks = ScopedFastFlag::new(&FFlag::DebugLuauAbortingChecks, true);

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        let fallback = b.fallback_block(0);
        let last = b.block(IrBlockKind::Internal);

        b.begin_block(block);

        let r1 = b.vm_reg(1);
        let ptr = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r1);

        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, r2, ptr);
        let r2 = b.vm_reg(2);
        let ttable = b.const_tag(TTABLE);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r2, ttable);

        b.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_READONLY, ptr, fallback);

        let r0 = b.vm_reg(0);
        let load = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r0);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, r2, load);
        let r2 = b.vm_reg(2);
        let ttable = b.const_tag(TTABLE);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r2, ttable);

        let r2 = b.vm_reg(2);
        let tnil = b.const_tag(TNIL);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r2, tnil);

        b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

        b.begin_block(fallback);
        let r1 = b.vm_reg(1);
        let fallback_ptr = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r1);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, r2, fallback_ptr);
        let r2 = b.vm_reg(2);
        let ttable = b.const_tag(TTABLE);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r2, ttable);
        b.inst_ir_cmd(IrCmd::CHECK_GC);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

        b.begin_block(last);
        let r0 = b.vm_reg(0);
        let three = b.const_int(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, three);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_2\n; in regs: R0, R1\n; out regs: R0, R1, R2\n   %0 = LOAD_POINTER R1\n   CHECK_READONLY %0, bb_fallback_1\n   STORE_TAG R2, tnil\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0\n; successors: bb_2\n; in regs: R0, R1\n; out regs: R0, R1, R2\n   %9 = LOAD_POINTER R1\n   STORE_POINTER R2, %9\n   STORE_TAG R2, ttable\n   CHECK_GC\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0, R1, R2\n   RETURN R0, 3i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
