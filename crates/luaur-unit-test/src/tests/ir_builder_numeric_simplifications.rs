//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3474:ir_builder_numeric_simplifications`
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
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_numeric_simplifications

#[cfg(test)]
#[test]
fn ir_builder_numeric_simplifications() {
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

        b.begin_block(block);
        let r0 = b.vm_reg(0);
        let value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r0);

        let zero = b.const_double(0.0);
        let sub = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_NUM, value, zero);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r1, sub);
        let neg_zero = b.const_double(-0.0);
        let add = b.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, value, neg_zero);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, add);

        let one = b.const_double(1.0);
        let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, value, one);
        let r3 = b.vm_reg(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r3, mul);
        let two = b.const_double(2.0);
        let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, value, two);
        let r4 = b.vm_reg(4);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r4, mul);
        let neg_one = b.const_double(-1.0);
        let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, value, neg_one);
        let r5 = b.vm_reg(5);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r5, mul);
        let three = b.const_double(3.0);
        let mul = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, value, three);
        let r6 = b.vm_reg(6);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r6, mul);

        let one = b.const_double(1.0);
        let div = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, value, one);
        let r7 = b.vm_reg(7);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r7, div);
        let neg_one = b.const_double(-1.0);
        let div = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, value, neg_one);
        let r8 = b.vm_reg(8);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r8, div);
        let thirty_two = b.const_double(32.0);
        let div = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, value, thirty_two);
        let r9 = b.vm_reg(9);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r9, div);
        let six = b.const_double(6.0);
        let div = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_NUM, value, six);
        let r10 = b.vm_reg(10);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r10, div);

        let r1 = b.vm_reg(1);
        let nine = b.const_int(9);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, nine);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   STORE_DOUBLE R1, %0\n   STORE_DOUBLE R2, %0\n   STORE_DOUBLE R3, %0\n   %7 = ADD_NUM %0, %0\n   STORE_DOUBLE R4, %7\n   %9 = UNM_NUM %0\n   STORE_DOUBLE R5, %9\n   %11 = MUL_NUM %0, 3\n   STORE_DOUBLE R6, %11\n   STORE_DOUBLE R7, %0\n   %15 = UNM_NUM %0\n   STORE_DOUBLE R8, %15\n   %17 = MUL_NUM %0, 0.03125\n   STORE_DOUBLE R9, %17\n   %19 = DIV_NUM %0, 6\n   STORE_DOUBLE R10, %19\n   RETURN R1, 9i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
