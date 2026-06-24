//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrBuilder.test.cpp:7695:ir_builder_to_dot`
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
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> function computeCfgInfo (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> method BytecodeBuilder::validate (Bytecode/src/BytecodeBuilder.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function toDotCfg (CodeGen/src/IrDump.cpp)
//!   - calls -> function toDotDjGraph (CodeGen/src/IrDump.cpp)
//!   - translates_to -> rust_item ir_builder_to_dot

#[cfg(test)]
#[test]
fn ir_builder_to_dot() {
    use crate::records::ir_builder_fixture::IrBuilderFixture;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::compute_cfg_info::compute_cfg_info;
    use luaur_code_gen::functions::to_dot::to_dot;
    use luaur_code_gen::functions::to_dot_cfg::to_dot_cfg;
    use luaur_code_gen::functions::to_dot_dj_graph::to_dot_dj_graph;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    const TNUMBER: u8 = 3;

    let mut fix = IrBuilderFixture::new();
    {
        let b = &mut fix.build;
        let entry = b.block(IrBlockKind::Internal);
        let a = b.block(IrBlockKind::Internal);
        let branch_b = b.block(IrBlockKind::Internal);
        let exit = b.block(IrBlockKind::Internal);

        b.begin_block(entry);
        let r0 = b.vm_reg(0);
        let tag = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, r0);
        let tnumber = b.const_tag(TNUMBER);
        b.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_EQ_TAG, tag, tnumber, a, branch_b);

        b.begin_block(a);
        let r2 = b.vm_reg(2);
        let r1 = b.vm_reg(1);
        let value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r2, value);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

        b.begin_block(branch_b);
        let r3 = b.vm_reg(3);
        let r1 = b.vm_reg(1);
        let value = b.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, r1);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, r3, value);
        b.inst_ir_cmd_ir_op(IrCmd::JUMP, exit);

        b.begin_block(exit);
        let r2 = b.vm_reg(2);
        let two = b.const_int(2);
        b.inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r2, two);
    }

    update_use_counts(&mut fix.build.function);
    compute_cfg_info(&mut fix.build.function);

    let _ = to_dot(&fix.build.function, true);
    let _ = to_dot_cfg(&fix.build.function);
    let _ = to_dot_dj_graph(&fix.build.function);
}
