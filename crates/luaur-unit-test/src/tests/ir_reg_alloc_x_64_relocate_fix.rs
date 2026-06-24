//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrRegAllocX64.test.cpp:32:ir_reg_alloc_x_64_relocate_fix`
//! Source: `tests/IrRegAllocX64.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrRegAllocX64.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
//! - incoming:
//!   - declares <- source_file tests/IrRegAllocX64.test.cpp
//! - outgoing:
//!   - type_ref -> record IrInst (CodeGen/include/Luau/IrData.h)
//!   - type_ref -> enum IrCmd (CodeGen/include/Luau/IrData.h)
//!   - calls -> method IrRegAllocX64::preserve (CodeGen/src/IrRegAllocX64.cpp)
//!   - calls -> method IrRegAllocX64Fixture::checkMatch (tests/IrRegAllocX64.test.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - translates_to -> rust_item ir_reg_alloc_x_64_relocate_fix

#[cfg(test)]
#[test]
fn ir_reg_alloc_x64_relocate_fix() {
    use crate::records::ir_reg_alloc_x_64_fixture::IrRegAllocX64Fixture;
    use alloc::string::String;
    use luaur_code_gen::enums::ir_cmd::IrCmd;
    use luaur_code_gen::records::ir_inst::IrInst;
    use luaur_code_gen::records::register_x_64::RegisterX64;

    let mut fixture = IrRegAllocX64Fixture::ir_reg_alloc_x_64_fixture();

    let mut ir_inst0 = IrInst::default();
    ir_inst0.cmd = IrCmd::LOAD_DOUBLE;
    ir_inst0.last_use = 2;
    fixture.function.instructions.push(ir_inst0);

    let mut ir_inst1 = IrInst::default();
    ir_inst1.cmd = IrCmd::LOAD_DOUBLE;
    ir_inst1.last_use = 2;
    fixture.function.instructions.push(ir_inst1);

    let reg0 = fixture.regs.take_reg(RegisterX64::rax, 0);
    fixture.function.instructions[0].reg_x64 = reg0;
    fixture.regs.preserve(&mut fixture.function.instructions[0]);

    let reg1 = fixture.regs.take_reg(RegisterX64::rax, 1);
    fixture.function.instructions[1].reg_x64 = reg1;
    fixture
        .regs
        .restore(&mut fixture.function.instructions[0], true);

    assert_eq!(fixture.function.instructions[0].reg_x64, RegisterX64::rax);
    assert!(fixture.function.instructions[1].spilled);

    fixture.check_match(String::from(
        "\n vmovsd      qword ptr [rsp+048h],rax\n vmovsd      qword ptr [rsp+050h],rax\n vmovsd      rax,qword ptr [rsp+048h]\n",
    ));
}
