//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:222:ir_builder_final_x_64_opt_eq_tag_2`
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
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method NativeModuleRef::swap (CodeGen/src/SharedCodeAllocator.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_final_x_64_opt_eq_tag_2

#[cfg(test)]
#[test]
fn ir_builder_final_x_64_opt_eq_tag_2() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::optimize_memory_operands_x_64_optimize_final_x_64_alt_b::optimize_memory_operands_x_64;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    let mut fix = IrBuilderFixture::new();
    let b = &mut fix.build;

    let block = b.block(IrBlockKind::Internal);
    let true_block = b.block(IrBlockKind::Internal);
    let false_block = b.block(IrBlockKind::Internal);

    b.begin_block(block);
    let r1 = b.vm_reg(1);
    let op_a = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r1);
    let r2 = b.vm_reg(2);
    let op_b = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r2);
    let r6 = b.vm_reg(6);
    b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r6, op_a);
    b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_EQ_TAG, op_a, op_b, true_block, false_block);

    b.begin_block(true_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    b.begin_block(false_block);
    let zero = b.const_uint(0);
    b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);

    update_use_counts(&mut fix.build.function);
    optimize_memory_operands_x_64(&mut fix.build.function);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_TAG R1\n   STORE_TAG R6, %0\n   JUMP_EQ_TAG R2, %0, bb_1, bb_2\n\nbb_1:\n   RETURN 0u\n\nbb_2:\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
