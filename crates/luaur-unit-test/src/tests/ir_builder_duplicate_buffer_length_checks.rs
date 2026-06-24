//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:4547:ir_builder_duplicate_buffer_length_checks`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::fallbackBlock (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::undef (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_duplicate_buffer_length_checks

#[cfg(test)]
#[test]
fn ir_builder_duplicate_buffer_length_checks() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;
    use luaur_common::FFlag;
    use luaur_vm::enums::lua_type::lua_Type;

    let _load_propagate_origin = ScopedFastFlag::new(&FFlag::LuauCodegenLoadPropagateOrigin, true);

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);
        let fallback = b.fallback_block(0);

        b.begin_block(block);

        let r0 = b.vm_reg(0);
        let source_buf = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r0);

        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r2, source_buf);
        let r2 = b.vm_reg(2);
        let buffer1 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r2);
        let index = b.const_int(12);
        let start = b.const_int(0);
        let size = b.const_int(4);
        let source = b.undef();
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CHECK_BUFFER_LEN,
            buffer1,
            index,
            start,
            size,
            source,
            fallback,
        );
        let index = b.const_int(12);
        let value = b.const_int(32);
        let tag = b.const_tag(lua_Type::LUA_TBUFFER as u8);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEI32, buffer1, index, value, tag);

        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r2, source_buf);
        let r2 = b.vm_reg(2);
        let buffer2 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r2);
        let index = b.const_int(8);
        let start = b.const_int(0);
        let size = b.const_int(4);
        let source = b.undef();
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CHECK_BUFFER_LEN,
            buffer2,
            index,
            start,
            size,
            source,
            fallback,
        );
        let index = b.const_int(8);
        let value = b.const_int(30);
        let tag = b.const_tag(lua_Type::LUA_TBUFFER as u8);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEI32, buffer2, index, value, tag);

        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r2, source_buf);
        let r2 = b.vm_reg(2);
        let buffer3 = b.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, r2);
        let index = b.const_int(16);
        let start = b.const_int(0);
        let size = b.const_int(4);
        let source = b.undef();
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CHECK_BUFFER_LEN,
            buffer3,
            index,
            start,
            size,
            source,
            fallback,
        );
        let index = b.const_int(16);
        let value = b.const_int(60);
        let tag = b.const_tag(lua_Type::LUA_TBUFFER as u8);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEI32, buffer3, index, value, tag);

        let index = b.const_int(16);
        let start = b.const_int(0);
        let size = b.const_int(2);
        let source = b.undef();
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CHECK_BUFFER_LEN,
            buffer3,
            index,
            start,
            size,
            source,
            fallback,
        );
        let index = b.const_int(16);
        let value = b.const_int(55);
        let tag = b.const_tag(lua_Type::LUA_TBUFFER as u8);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEI16, buffer3, index, value, tag);

        let r1 = b.vm_reg(1);
        let index = b.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, r1);
        let start = b.const_int(0);
        let size = b.const_int(2);
        let source = b.undef();
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CHECK_BUFFER_LEN,
            buffer3,
            index,
            start,
            size,
            source,
            fallback,
        );
        let value = b.const_int(1);
        let tag = b.const_tag(lua_Type::LUA_TBUFFER as u8);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEI16, buffer3, index, value, tag);
        let start = b.const_int(0);
        let size = b.const_int(2);
        let source = b.undef();
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CHECK_BUFFER_LEN,
            buffer3,
            index,
            start,
            size,
            source,
            fallback,
        );
        let value = b.const_int(2);
        let tag = b.const_tag(lua_Type::LUA_TBUFFER as u8);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::BUFFER_WRITEI16, buffer3, index, value, tag);

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
    let expected = "\nbb_0:\n   %0 = LOAD_TVALUE R0\n   STORE_TVALUE R2, %0\n   %2 = LOAD_POINTER R0\n   CHECK_BUFFER_LEN %2, 12i, -4i, 8i, undef, bb_fallback_1\n   BUFFER_WRITEI32 %2, 12i, 32i, tbuffer\n   BUFFER_WRITEI32 %2, 8i, 30i, tbuffer\n   BUFFER_WRITEI32 %2, 16i, 60i, tbuffer\n   BUFFER_WRITEI16 %2, 16i, 55i, tbuffer\n   %15 = LOAD_INT R1\n   CHECK_BUFFER_LEN %2, %15, 0i, 2i, undef, bb_fallback_1\n   BUFFER_WRITEI16 %2, %15, 1i, tbuffer\n   BUFFER_WRITEI16 %2, %15, 2i, tbuffer\n   RETURN R1, 1u\n\nbb_fallback_1:\n   RETURN R0, 1u\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
