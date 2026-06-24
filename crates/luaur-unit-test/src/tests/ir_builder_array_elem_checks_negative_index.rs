//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4500:ir_builder_array_elem_checks_negative_index`
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
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_array_elem_checks_negative_index

#[cfg(test)]
#[test]
fn ir_builder_array_elem_checks_negative_index() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        let fallback = b.fallback_block(0);

        b.begin_block(block);

        let r1 = b.vm_reg(1);
        let table1 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r1);
        let zero = b.const_int(0);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_ARRAY_SIZE, table1, zero, fallback);
        let zero = b.const_int(0);
        let elem1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GET_ARR_ADDR, table1, zero);
        let zero = b.const_int(0);
        let value1 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, elem1, zero);
        let r3 = b.vm_reg(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r3, value1);

        let minus_one = b.const_int(-1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_ARRAY_SIZE, table1, minus_one, fallback);
        let minus_one = b.const_int(-1);
        let elem2 = b.inst_ir_cmd_ir_op_ir_op(IrCmd::GET_ARR_ADDR, table1, minus_one);
        let zero = b.const_int(0);
        let value1b = b.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, elem2, zero);
        let r4 = b.vm_reg(4);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r4, value1b);

        let r3 = b.vm_reg(3);
        let a = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r3);
        let r4 = b.vm_reg(4);
        let b_value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r4);
        let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, a, b_value);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, sum);

        let r2 = b.vm_reg(2);
        let one = b.const_uint(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, one);

        b.begin_block(fallback);
        let r0 = b.vm_reg(0);
        let one = b.const_uint(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_POINTER R1\n   CHECK_ARRAY_SIZE %0, 0i, bb_fallback_1\n   %2 = GET_ARR_ADDR %0, 0i\n   %3 = LOAD_TVALUE %2, 0i\n   STORE_TVALUE R3, %3\n   JUMP bb_fallback_1\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
