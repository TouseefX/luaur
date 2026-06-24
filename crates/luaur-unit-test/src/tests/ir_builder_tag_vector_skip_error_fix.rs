//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4723:ir_builder_tag_vector_skip_error_fix`
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
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_tag_vector_skip_error_fix

#[cfg(test)]
#[test]
fn ir_builder_tag_vector_skip_error_fix() {
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
        let a = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r0);
        let r1 = b.vm_reg(1);
        let b_value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r1);

        let mul_vec = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_VEC, a, b_value);
        let mul = b.inst_ir_cmd_ir_op(IrCmd::TAG_VECTOR, mul_vec);

        let add_vec = b.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_VEC, mul, mul);
        let t1 = b.inst_ir_cmd_ir_op(IrCmd::TAG_VECTOR, add_vec);
        let sub_vec = b.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_VEC, mul, mul);
        let t2 = b.inst_ir_cmd_ir_op(IrCmd::TAG_VECTOR, sub_vec);

        let unm_vec = b.inst_ir_cmd_ir_op(IrCmd::UNM_VEC, t2);
        let div_vec = b.inst_ir_cmd_ir_op_ir_op(IrCmd::DIV_VEC, t1, unm_vec);
        let t3 = b.inst_ir_cmd_ir_op(IrCmd::TAG_VECTOR, div_vec);

        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r0, t3);
        let r0 = b.vm_reg(0);
        let one = b.const_uint(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::Yes);
    let expected = "\nbb_0:                                                       ; useCount: 0\n   %0 = LOAD_TVALUE R0                                       ; useCount: 1, lastUse: %0\n   %1 = LOAD_TVALUE R1                                       ; useCount: 1, lastUse: %0\n   %2 = MUL_VEC %0, %1                                       ; useCount: 4, lastUse: %0\n   %4 = ADD_VEC %2, %2                                       ; useCount: 1, lastUse: %0\n   %6 = SUB_VEC %2, %2                                       ; useCount: 1, lastUse: %0\n   %8 = UNM_VEC %6                                           ; useCount: 1, lastUse: %0\n   %9 = DIV_VEC %4, %8                                       ; useCount: 1, lastUse: %0\n   %10 = TAG_VECTOR %9                                       ; useCount: 1, lastUse: %0\n   STORE_TVALUE R0, %10                                      ; %11\n   RETURN R0, 1u                                             ; %12\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
