//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4149:ir_builder_duplicate_hash_slot_checks_avoid_nil`
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
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_duplicate_hash_slot_checks_avoid_nil

#[cfg(test)]
#[test]
fn ir_builder_duplicate_hash_slot_checks_avoid_nil() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        let fallback = b.fallback_block(0);

        b.begin_block(block);

        let r1 = b.vm_reg(1);
        let table1 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r1);
        let three = b.const_uint(3);
        let k1 = b.vm_const(1);
        let slot1 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_SLOT_NODE_ADDR, table1, three, k1);
        let k1 = b.vm_const(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_SLOT_MATCH, slot1, k1, fallback);
        let zero = b.const_int(0);
        let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, slot1, zero);
        let r3 = b.vm_reg(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r3, value1);

        let r2 = b.vm_reg(2);
        let table2 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r2);
        let six = b.const_uint(6);
        let k1 = b.vm_const(1);
        let slot2 = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_SLOT_NODE_ADDR, table2, six, k1);
        let k1 = b.vm_const(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_SLOT_MATCH, slot2, k1, fallback);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_READONLY, table2, fallback);

        let r4 = b.vm_reg(4);
        let tnil = b.const_tag(TNIL);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r4, tnil);
        let r4 = b.vm_reg(4);
        let value_nil = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r4);
        let zero = b.const_int(0);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::STORE_TVALUE, slot2, value_nil, zero);

        let eight = b.const_uint(8);
        let k1 = b.vm_const(1);
        let slot1b = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_SLOT_NODE_ADDR, table1, eight, k1);
        let k1 = b.vm_const(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_SLOT_MATCH, slot1b, k1, fallback);
        let zero = b.const_int(0);
        let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, slot1b, zero);
        let r3 = b.vm_reg(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r3, value1b);

        let eleven = b.const_uint(11);
        let k1 = b.vm_const(1);
        let slot2b = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_SLOT_NODE_ADDR, table2, eleven, k1);
        let k1 = b.vm_const(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_SLOT_MATCH, slot2b, k1, fallback);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_READONLY, table2, fallback);

        let tnumber = b.const_tag(TNUMBER);
        let one_double = b.const_double(1.0);
        let zero = b.const_int(0);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
            IrCmd::STORE_SPLIT_TVALUE,
            slot2b,
            tnumber,
            one_double,
            zero,
        );

        let r3 = b.vm_reg(3);
        let two = b.const_uint(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r3, two);

        b.begin_block(fallback);
        let r1 = b.vm_reg(1);
        let two = b.const_uint(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   %1 = GET_SLOT_NODE_ADDR %0, 3u, K1\n   CHECK_SLOT_MATCH %1, K1, bb_fallback_1\n   %3 = LOAD_TVALUE %1, 0i\n   STORE_TVALUE R3, %3\n   %5 = LOAD_POINTER R2\n   %6 = GET_SLOT_NODE_ADDR %5, 6u, K1\n   CHECK_SLOT_MATCH %6, K1, bb_fallback_1\n   CHECK_READONLY %5, bb_fallback_1\n   STORE_TAG R4, tnil\n   %10 = LOAD_TVALUE R4, 0i, tnil\n   STORE_TVALUE %6, %10, 0i\n   CHECK_NODE_VALUE %1, bb_fallback_1\n   %14 = LOAD_TVALUE %1, 0i\n   STORE_TVALUE R3, %14\n   CHECK_NODE_VALUE %6, bb_fallback_1\n   STORE_SPLIT_TVALUE %6, tnumber, 1, 0i\n   RETURN R3, 2u\n\nbb_fallback_1:\n   RETURN R1, 2u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
