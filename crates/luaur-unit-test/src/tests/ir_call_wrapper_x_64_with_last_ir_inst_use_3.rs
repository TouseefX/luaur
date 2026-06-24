//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:432:ir_call_wrapper_x_64_with_last_ir_inst_use_3`
//! Source: `tests/IrCallWrapperX64.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
//!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
//! - incoming:
//!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
//! - outgoing:
//!   - type_ref -> record IrInst (CodeGen/include/Luau/IrData.h)
//!   - type_ref -> record IrOp (CodeGen/include/Luau/IrData.h)
//!   - type_ref -> enum IrOpKind (CodeGen/include/Luau/IrData.h)
//!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
//!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - translates_to -> rust_item ir_call_wrapper_x_64_with_last_ir_inst_use_3

#[cfg(test)]
#[test]
fn ir_call_wrapper_x_64_with_last_ir_inst_use3() {
    use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
    use luaur_code_gen::enums::ir_op_kind::IrOpKind;
    use luaur_code_gen::enums::size_x_64::SizeX64;
    use luaur_code_gen::records::ir_inst::IrInst;
    use luaur_code_gen::records::ir_op::IrOp;
    use luaur_code_gen::records::operand_x_64::qword;
    use luaur_code_gen::records::register_x_64::RegisterX64;

    let mut fixture = IrCallWrapperX64Fixture::windows();
    let mut ir_inst1 = IrInst::default();
    let ir_op1 = IrOp::ir_op_ir_op_kind_u32(IrOpKind::Inst, 0);
    ir_inst1.reg_x64 = fixture.regs.take_reg(RegisterX64::xmm0, ir_op1.index());
    ir_inst1.last_use = 1;
    fixture.function.instructions.push(ir_inst1.clone());
    fixture.call_wrap.inst_idx = ir_inst1.last_use;

    fixture.call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::xmmword,
        ir_inst1.reg_x64.into(),
        ir_op1,
    );
    fixture.call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::xmmword,
        ir_inst1.reg_x64.into(),
        ir_op1,
    );
    fixture.call(qword.operator_bracket(RegisterX64::r12.into()));

    fixture.check_match(String::from(
        r#"
 vmovsd      xmm1,xmm0,xmm0
 call        qword ptr [r12]
"#,
    ));
}
