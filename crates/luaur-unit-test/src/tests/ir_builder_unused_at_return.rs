//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:5993:ir_builder_unused_at_return`
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
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - calls -> function markDeadStoresInBlockChains (CodeGen/src/OptimizeDeadStore.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_unused_at_return

#[cfg(test)]
#[test]
fn ir_builder_unused_at_return() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::mark_dead_stores_in_block_chains::mark_dead_stores_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNIL: u8 = 0;
    const TBOOLEAN: u8 = 1;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);

        b.begin_block(entry);
        let r1 = b.vm_reg(1);
        let one = b.const_double(1.0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r1, one);
        let r1 = b.vm_reg(1);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, tnumber);
        let r2 = b.vm_reg(2);
        let four = b.const_int(4);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r2, four);
        let r2 = b.vm_reg(2);
        let tboolean = b.const_tag(TBOOLEAN);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r2, tboolean);
        let r4 = b.vm_reg(4);
        let tnumber = b.const_tag(TNUMBER);
        let two = b.const_double(2.0);
        b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::STORE_SPLIT_TVALUE, r4, tnumber, two);

        let r0 = b.vm_reg(0);
        let some_tv = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r0);
        let r5 = b.vm_reg(5);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r5, some_tv);

        let r6 = b.vm_reg(6);
        let tnil = b.const_tag(TNIL);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r6, tnil);

        let r0 = b.vm_reg(0);
        let one = b.const_int(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, one);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);
    mark_dead_stores_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n; in regs: R0\n   RETURN R0, 1i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
