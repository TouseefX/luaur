//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4687:ir_builder_buffer_length_checks_integer_match_2`
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
//!   - calls -> method IrBuilder::constDouble (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_buffer_length_checks_integer_match_2

#[cfg(test)]
#[test]
fn ir_builder_buffer_length_checks_integer_match_2() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;
    use luaur_vm::enums::lua_type::lua_Type;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        let fallback = b.fallback_block(0);

        b.begin_block(block);
        let r0 = b.vm_reg(0);
        let buffer = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r0);

        let r1 = b.vm_reg(1);
        let value = b.const_double(1000000.0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r1, value);
        let r1 = b.vm_reg(1);
        let tag = b.const_tag(lua_Type::LUA_TNUMBER as u8);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, tag);

        let r1 = b.vm_reg(1);
        let value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r1);
        let squared = b.inst_ir_cmd_ir_op_ir_op(IrCmd::MUL_NUM, value, value);
        let base = b.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT, squared);
        let start = b.const_int(0);
        let size = b.const_int(4);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CHECK_BUFFER_LEN,
            buffer,
            base,
            start,
            size,
            squared,
            fallback,
        );
        let index = b.const_int(0);
        let value = b.const_int(32);
        let tag = b.const_tag(lua_Type::LUA_TBUFFER as u8);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEI32, buffer, index, value, tag);
        let r1 = b.vm_reg(1);
        let one = b.const_uint(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, one);

        b.begin_block(fallback);
        let r0 = b.vm_reg(0);
        let one = b.const_uint(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_DOUBLE R1, 1000000\n   STORE_TAG R1, tnumber\n   JUMP bb_fallback_1\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
