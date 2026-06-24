//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6513:ir_builder_full_store_has_to_be_observable_from_fallbacks`
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
//!   - calls -> method Path::last (Analysis/src/TypePath.cpp)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - translates_to -> rust_item ir_builder_full_store_has_to_be_observable_from_fallbacks

#[cfg(test)]
#[test]
fn ir_builder_full_store_has_to_be_observable_from_fallbacks() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNUMBER: u8 = 3;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);
        let fallback = b.fallback_block(0);
        let last = b.block(IrBlockKind::Internal);

        b.begin_block(entry);
        let sixteen = b.const_uint(16);
        let thirty_two = b.const_uint(32);
        let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_TABLE, sixteen, thirty_two);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, r1, table);
        let r1 = b.vm_reg(1);
        let ttable = b.const_tag(TTABLE);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, ttable);
        b.inst_ir_cmd_ir_op(IrCmd::CHECK_SAFE_ENV, fallback);
        let sixteen = b.const_uint(16);
        let thirty_two = b.const_uint(32);
        let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_TABLE, sixteen, thirty_two);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, r1, table);
        let r1 = b.vm_reg(1);
        let ttable = b.const_tag(TTABLE);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, ttable);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

        b.begin_block(fallback);
        b.inst_ir_cmd(IrCmd::CHECK_GC);
        let r1 = b.vm_reg(1);
        let tnumber = b.const_tag(TNUMBER);
        let one = b.const_double(1.0);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::STORE_SPLIT_TVALUE, r1, tnumber, one);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, last);

        b.begin_block(last);
        let r0 = b.vm_reg(0);
        let two = b.const_int(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_fallback_1, bb_2\n; in regs: R0\n; out regs: R0, R1\n   CHECK_SAFE_ENV bb_fallback_1\n   %4 = NEW_TABLE 16u, 32u\n   STORE_SPLIT_TVALUE R1, ttable, %4\n   JUMP bb_2\n\nbb_fallback_1:\n; predecessors: bb_0\n; successors: bb_2\n; in regs: R0\n; out regs: R0, R1\n   CHECK_GC\n   STORE_SPLIT_TVALUE R1, tnumber, 1\n   JUMP bb_2\n\nbb_2:\n; predecessors: bb_0, bb_fallback_1\n; in regs: R0, R1\n   RETURN R0, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
