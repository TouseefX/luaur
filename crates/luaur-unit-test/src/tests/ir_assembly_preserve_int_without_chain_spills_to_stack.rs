//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrAssembly.test.cpp:227:ir_assembly_preserve_int_without_chain_spills_to_stack`
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
//!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> enum IrBlockKind (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::beginBlock (CodeGen/src/IrBuilder.cpp)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrBuilder::vmReg (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constUint (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method IrBuilder::constInt (CodeGen/src/IrBuilder.cpp)
//!   - calls -> function updateUseCounts (CodeGen/src/IrAnalysis.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - calls -> method AssemblyBuilderX64::align (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderX64::ud2 (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - calls -> method AssemblyBuilderX64::vaddsd (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderX64::vcvttsd2si (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item ir_assembly_preserve_int_without_chain_spills_to_stack

#[cfg(test)]
#[test]
fn ir_assembly_preserve_int_without_chain_spills_to_stack() {
    use crate::records::ir_assembly_fixture::IrAssemblyFixture;
    use luaur_code_gen::enums::ir_block_kind::IrBlockKind;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::functions::update_use_counts::update_use_counts;

    let mut fixture = IrAssemblyFixture::ir_assembly_fixture();
    let entry = fixture.build.block(IrBlockKind::Internal);

    fixture.build.begin_block(entry);
    let r1 = fixture.build.vm_reg(1);
    let a = fixture.build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r1);
    let r2 = fixture.build.vm_reg(2);
    let b = fixture.build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, r2);
    let sum = fixture.build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, a, b);
    let i = fixture.build.inst_ir_cmd_ir_op(IrCmd::NUM_TO_INT, sum);
    let pcpos = fixture.build.const_uint(0);
    fixture.build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos);
    let r3 = fixture.build.vm_reg(3);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, r3, i);
    let r0 = fixture.build.vm_reg(0);
    let count = fixture.build.const_int(0);
    fixture
        .build
        .inst_ir_cmd_ir_op_ir_op(IrCmd::RETURN, r0, count);
    update_use_counts(&mut fixture.build.function);

    let expected = r#"
; align 32 using ud2
bb_0:
.L11:
  %0 = LOAD_DOUBLE R1
 vmovsd      xmm0,qword ptr [r14+010h]
  %2 = ADD_NUM %0, R2
 vaddsd      xmm0,xmm0,qword ptr [r14+020h]
  %3 = NUM_TO_INT %2
 vcvttsd2si  eax,xmm0
  INTERRUPT 0u
 mov         dword ptr [rsp+048h],eax
 mov         rax,qword ptr [r15+<offset>]
 cmp         qword ptr [rax+<offset>],0
 jne         .L12
.L13:
  STORE_INT R3, %3
 mov         eax,dword ptr [rsp+048h]
 mov         dword ptr [r14+030h],eax
  RETURN R0, 0i
 lea         rdi,[r14-010h]
 xor         ecx,ecx
 jmp         .L7

"#;

    assert_eq!(expected, format!("\n{}", fixture.lower()));
}
