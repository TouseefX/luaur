//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:3523:ir_builder_cmp_tag_simplification`
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
//!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCondition (CodeGen/include/Luau/IrData.h)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function constPropInBlockChains (CodeGen/src/OptimizeConstProp.cpp)
//!   - type_ref -> enum IncludeUseInfo (CodeGen/include/Luau/CodeGenOptions.h)
//!   - translates_to -> rust_item ir_builder_cmp_tag_simplification

#[cfg(test)]
#[test]
fn ir_builder_cmp_tag_simplification() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::include_use_info::IncludeUseInfo;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::enums::ir_condition::IrCondition;
    use luaur_code_gen::functions::const_prop_in_block_chains::const_prop_in_block_chains;
    use luaur_code_gen::functions::to_string_ir_dump_alt_g::to_string;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNIL: u8 = 0;
    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let block = b.block(IrBlockKind::Internal);

        b.begin_block(block);

        let eq = b.cond(IrCondition::Equal);
        let neq = b.cond(IrCondition::NotEqual);

        let tnil = b.const_tag(TNIL);
        let tnumber = b.const_tag(TNUMBER);
        let cmp = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_TAG, tnil, tnumber, eq);
        let r0 = b.vm_reg(0);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r0, cmp);
        let tnumber1 = b.const_tag(TNUMBER);
        let tnumber2 = b.const_tag(TNUMBER);
        let cmp = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_TAG, tnumber1, tnumber2, eq);
        let r1 = b.vm_reg(1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r1, cmp);
        let tnil = b.const_tag(TNIL);
        let tnumber = b.const_tag(TNUMBER);
        let cmp = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_TAG, tnil, tnumber, neq);
        let r2 = b.vm_reg(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r2, cmp);
        let tnumber1 = b.const_tag(TNUMBER);
        let tnumber2 = b.const_tag(TNUMBER);
        let cmp = b.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_TAG, tnumber1, tnumber2, neq);
        let r3 = b.vm_reg(3);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r3, cmp);

        let r0 = b.vm_reg(0);
        let four = b.const_int(4);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, four);
    }

    update_use_counts(&mut fix.build.function);
    const_prop_in_block_chains(&mut fix.build);

    let dump = to_string(&mut fix.build.function, IncludeUseInfo::No);
    let expected = "\nbb_0:\n   STORE_INT R0, 0i\n   STORE_INT R1, 1i\n   STORE_INT R2, 1i\n   STORE_INT R3, 0i\n   RETURN R0, 4i\n\n";
    assert_eq!(format!("\n{}", dump), expected);
}
