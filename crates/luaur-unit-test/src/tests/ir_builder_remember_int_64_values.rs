//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:2488:ir_builder_remember_int_64_values`
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
//!   - calls -> method IrBuilder::constInt64 (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function load (Config/src/LuauConfig.cpp)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_remember_int_64_values

#[cfg(test)]
#[test]
fn ir_builder_remember_int_64_values() {
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
        let c42 = b.const_int_64(42);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r0, c42);

        let r0 = b.vm_reg(0);
        let loaded = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r0);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r1, loaded);

        let r0 = b.vm_reg(0);
        let c42 = b.const_int_64(42);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r0, c42);

        let r5 = b.vm_reg(5);
        let unknown = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r5);
        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r0, unknown);

        let r0 = b.vm_reg(0);
        let loaded = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, r0);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT64, r2, loaded);

        let zero = b.const_uint(0);
        b.inst_ir_cmd_ir_op(IrCmd::RETURN, zero);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT64 R0, 42i\n   STORE_INT64 R1, 42i\n   %4 = LOAD_INT64 R5\n   STORE_INT64 R0, %4\n   STORE_INT64 R2, %4\n   RETURN 0u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
