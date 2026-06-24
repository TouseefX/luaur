//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2430:ir_builder_remember_tags_and_values`
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
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_remember_tags_and_values

#[cfg(test)]
#[test]
fn ir_builder_remember_tags_and_values() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);

        b.begin_block(block);

        let r0 = b.vm_reg(0);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r0, tnumber);
        let r1 = b.vm_reg(1);
        let ten = b.const_int(10);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r1, ten);
        let r2 = b.vm_reg(2);
        let half = b.const_double(0.5);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, half);

        let r0 = b.vm_reg(0);
        let tag = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r0);
        let r3 = b.vm_reg(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r3, tag);
        let r1 = b.vm_reg(1);
        let int = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, r1);
        let r4 = b.vm_reg(4);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r4, int);
        let r2 = b.vm_reg(2);
        let double = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r2);
        let r5 = b.vm_reg(5);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r5, double);

        let r0 = b.vm_reg(0);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r0, tnumber);
        let r1 = b.vm_reg(1);
        let ten = b.const_int(10);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r1, ten);
        let r2 = b.vm_reg(2);
        let half = b.const_double(0.5);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, half);

        let r6 = b.vm_reg(6);
        let tag = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r6);
        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r0, tag);
        let r7 = b.vm_reg(7);
        let int = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, r7);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r1, int);
        let r8 = b.vm_reg(8);
        let double = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r8);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, double);

        let r0 = b.vm_reg(0);
        let tag = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r0);
        let r9 = b.vm_reg(9);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r9, tag);
        let r1 = b.vm_reg(1);
        let int = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, r1);
        let r10 = b.vm_reg(10);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r10, int);
        let r2 = b.vm_reg(2);
        let double = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r2);
        let r11 = b.vm_reg(11);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r11, double);

        let zero = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_TAG R0, tnumber\n   STORE_INT R1, 10i\n   STORE_DOUBLE R2, 0.5\n   STORE_TAG R3, tnumber\n   STORE_INT R4, 10i\n   STORE_DOUBLE R5, 0.5\n   %12 = LOAD_TAG R6\n   STORE_TAG R0, %12\n   %14 = LOAD_INT R7\n   STORE_INT R1, %14\n   %16 = LOAD_DOUBLE R8\n   STORE_DOUBLE R2, %16\n   %18 = LOAD_TAG R0\n   STORE_TAG R9, %18\n   STORE_INT R10, %14\n   STORE_DOUBLE R11, %16\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
