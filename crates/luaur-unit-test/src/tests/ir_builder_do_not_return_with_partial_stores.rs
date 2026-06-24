//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:6785:ir_builder_do_not_return_with_partial_stores`
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
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - calls -> function successors (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function predecessors (CodeGen/src/IrAnalysis.cpp)
//!   - translates_to -> rust_item ir_builder_do_not_return_with_partial_stores

#[cfg(test)]
#[test]
fn ir_builder_do_not_return_with_partial_stores() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::enums::ir_condition::IrCondition;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TBOOLEAN: u8 = 1;
    const TTABLE: u8 = 7;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);
        let success = b.block(IrBlockKind::Internal);
        let fail = b.block(IrBlockKind::Internal);
        let exit = b.block(IrBlockKind::Internal);

        b.begin_block(entry);
        let zero_u = b.const_uint(0);
        let zero_u_2 = b.const_uint(0);
        let table = b.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_TABLE, zero_u, zero_u_2);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, r1, table);
        let r1 = b.vm_reg(1);
        let ttable = b.const_tag(TTABLE);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, ttable);
        let big = b.const_double(1e20);
        let to_uint = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_UINT, big);
        let four = b.const_int(4);
        let bit_and = b.inst_ir_cmd_ir_op_ir_op(IrCmd::BITAND_UINT, to_uint, four);
        let zero = b.const_int(0);
        let equal = b.cond(IrCondition::Equal);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::JUMP_CMP_INT,
            bit_and,
            zero,
            equal,
            success,
            fail,
        );

        b.begin_block(success);
        let r1 = b.vm_reg(1);
        let zero = b.const_int(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r1, zero);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

        b.begin_block(fail);
        let r1 = b.vm_reg(1);
        let one = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r1, one);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

        b.begin_block(exit);
        let r1 = b.vm_reg(1);
        let tboolean = b.const_tag(TBOOLEAN);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, tboolean);
        let r0 = b.vm_reg(0);
        let one = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; successors: bb_1, bb_2\n; in regs: R0\n; out regs: R0\n   %3 = NUM_TO_UINT 1e+20\n   %4 = BITAND_UINT %3, 4i\n   JUMP_CMP_INT %4, 0i, eq, bb_1, bb_2\n\nbb_1:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R0\n; out regs: R0\n   STORE_INT R1, 0i\n   JUMP bb_3\n\nbb_2:\n; predecessors: bb_0\n; successors: bb_3\n; in regs: R0\n; out regs: R0\n   STORE_INT R1, 1i\n   JUMP bb_3\n\nbb_3:\n; predecessors: bb_1, bb_2\n; in regs: R0\n   STORE_TAG R1, tboolean\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
