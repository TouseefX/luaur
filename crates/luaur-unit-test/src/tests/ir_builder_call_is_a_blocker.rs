//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5537:ir_builder_call_is_a_blocker`
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
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_call_is_a_blocker

#[cfg(test)]
#[test]
fn ir_builder_call_is_a_blocker() {
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
        let entry = b.block(IrBlockKind::Internal);

        b.begin_block(entry);
        let r0 = b.vm_reg(0);
        let op1 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r0);
        let r1 = b.vm_reg(1);
        let one_params = b.const_int(1);
        let r2 = b.vm_reg(2);
        let one_results = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::CALL, r1, one_params, r2, one_results);
        let r0 = b.vm_reg(0);
        let op2 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r0);
        let sum = b.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, op1, op2);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r1, sum);
        let r1 = b.vm_reg(1);
        let two = b.const_int(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, two);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   %0 = LOAD_DOUBLE R0\n   CALL R1, 1i, R2, 1i\n   %2 = LOAD_DOUBLE R0\n   %3 = ADD_NUM %0, %2\n   STORE_DOUBLE R1, %3\n   RETURN R1, 2i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
