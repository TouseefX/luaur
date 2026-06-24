//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrAssembly.test.cpp:272:ir_assembly_dse_hint_materializes_int_into_dead_vm_reg`
//! Source: `tests/IrAssembly.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrAssembly.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file CodeGen/include/Luau/IrAnalysis.h
//!   - includes -> source_file CodeGen/include/Luau/IrBuilder.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/IrAssembly.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constTag (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - calls -> method AssemblyBuilderX64::align (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderX64::ud2 (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - calls -> method AssemblyBuilderX64::vcvttsd2si (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderX64::vaddsd (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderX64::vcvtsi2sd (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderX64::vmovups (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item ir_assembly_dse_hint_materializes_int_into_dead_vm_reg

#[cfg(test)]
#[test]
fn ir_assembly_dse_hint_materializes_int_into_dead_vm_reg() {
    use crate::records::ir_assembly_fixture::IrAssemblyFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;
    use luaur_common::FFlag;

    let _luau_codegen_dse_restore_hints =
        ScopedFastFlag::new(&FFlag::LuauCodegenDseRestoreHints, true);
    let _luau_codegen_forward_rematerialize =
        ScopedFastFlag::new(&FFlag::LuauCodegenForwardRematerialize, true);

    let mut fixture = IrAssemblyFixture::ir_assembly_fixture();
    let entry = fixture.build.block(IrBlockKind::Internal);
    fixture.build.begin_block(entry);

    let r1 = fixture.build.vm_reg(1);
    let d = fixture.build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r1);
    let i = fixture.build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT, d);

    let doubled = fixture.build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, d, d);
    let r1 = fixture.build.vm_reg(1);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r1, doubled);
    let r1 = fixture.build.vm_reg(1);
    let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r1, tnumber);

    let roundtrip = fixture.build.inst_ir_cmd_ir_op(IrCmd::INT_TO_NUM, i);
    let r4 = fixture.build.vm_reg(4);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r4, roundtrip);
    let r4 = fixture.build.vm_reg(4);
    let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r4, tnumber);

    let pcpos = fixture.build.const_uint(0);
    fixture.build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);

    let r2 = fixture.build.vm_reg(2);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, r2, roundtrip);
    let r2 = fixture.build.vm_reg(2);
    let tnumber = fixture.build.const_tag(IrAssemblyFixture::TNUMBER);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, r2, tnumber);

    let r1 = fixture.build.vm_reg(1);
    let count = fixture.build.const_int(2);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r1, count);
    update_use_counts(&mut fixture.build.function);

    let expected = r#"
; align 32 using ud2
bb_0:
.L11:
  %0 = LOAD_DOUBLE R1
 vmovsd      xmm0,qword ptr [r14+010h]
  %1 = NUM_TO_INT %0
 vcvttsd2si  eax,xmm0
  %2 = ADD_NUM %0, %0
 vaddsd      xmm0,xmm0,xmm0
  STORE_DOUBLE R1, %2
 vmovsd      qword ptr [r14+010h],xmm0
  STORE_TAG R1, tnumber
 mov         dword ptr [r14+01Ch],3
  %5 = INT_TO_NUM %1
 vcvtsi2sd   xmm0,xmm0,eax
  INTERRUPT 0u
 vmovsd      qword ptr [r14+040h],xmm0
 mov         dword ptr [r14+04Ch],0
 mov         rax,qword ptr [r15+<offset>]
 cmp         qword ptr [rax+<offset>],0
 jne         .L12
.L13:
  STORE_DOUBLE R2, %5
 vmovsd      xmm0,qword ptr [r14+040h]
 vmovsd      qword ptr [r14+020h],xmm0
  STORE_TAG R2, tnumber
 mov         dword ptr [r14+02Ch],3
  RETURN R1, 2i
 lea         rdi,[r14-010h]
 vmovups     xmm0,xmmword ptr [r14+010h]
 vmovups     xmmword ptr [rdi],xmm0
 vmovups     xmm0,xmmword ptr [r14+020h]
 vmovups     xmmword ptr [rdi+010h],xmm0
 add         rdi,20h
 mov         ecx,2
 jmp         .L7

"#;

    assert_eq!(expected, format!("\n{}", fixture.lower()));
}
