//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4922:ir_builder_do_not_produce_invalid_split_store_3`
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
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::vmExit (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function load (Config/src/LuauConfig.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_do_not_produce_invalid_split_store_3

#[cfg(test)]
#[test]
fn ir_builder_do_not_produce_invalid_split_store_3() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;
    use luaur_vm::enums::lua_type::lua_Type;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);

        b.begin_block(entry);

        let r0 = b.vm_reg(0);
        let two = b.const_int(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r0, two);

        let r0 = b.vm_reg(0);
        let tag = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r0);
        let tnumber = b.const_tag(lua_Type::LUA_TNUMBER as u8);
        let exit = b.vm_exit(1);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag, tnumber, exit);

        let r2 = b.vm_reg(2);
        let tnumber = b.const_tag(lua_Type::LUA_TNUMBER as u8);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r2, tnumber);
        let r0 = b.vm_reg(0);
        let double = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r0);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, double);

        let r0 = b.vm_reg(0);
        let value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r0);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r1, value);
        let r1 = b.vm_reg(1);
        let two = b.const_int(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r1, two);
        let r1 = b.vm_reg(1);
        let tboolean = b.const_tag(lua_Type::LUA_TBOOLEAN as u8);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, tboolean);
        let r1 = b.vm_reg(1);
        let one = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 2i\n   %1 = LOAD_TAG R0\n   CHECK_TAG %1, tnumber, exit(1)\n   STORE_TAG R2, tnumber\n   %4 = LOAD_DOUBLE R0\n   STORE_DOUBLE R2, %4\n   %6 = LOAD_TVALUE R0, 0i, tnumber\n   STORE_TVALUE R1, %6\n   STORE_TAG R1, tboolean\n   RETURN R1, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
