//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:531:ir_call_wrapper_x_64_suggested_conflict_with_reserved`
//! Source: `tests/IrCallWrapperX64.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
//!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
//! - incoming:
//!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
//! - outgoing:
//!   - type_ref -> record ScopedRegX64 (CodeGen/include/Luau/IrRegAllocX64.h)
//!   - type_ref -> record IrCallWrapperX64 (CodeGen/include/Luau/IrCallWrapperX64.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
//!   - type_ref -> record RegisterX64 (CodeGen/include/Luau/RegisterX64.h)
//!   - calls -> method IrCallWrapperX64::suggestNextArgumentRegister (CodeGen/src/IrCallWrapperX64.cpp)
//!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
//!   - translates_to -> rust_item ir_call_wrapper_x_64_suggested_conflict_with_reserved

#[cfg(test)]
#[test]
fn ir_call_wrapper_x_64_suggested_conflict_with_reserved() {
    use crate::records::ir_call_wrapper_x_64_fixture_system_v::IrCallWrapperX64FixtureSystemV;
    use luaur_code_gen::enums::size_x_64::SizeX64;
    use luaur_code_gen::records::ir_call_wrapper_x_64::IrCallWrapperX64;
    use luaur_code_gen::records::ir_op::IrOp;
    use luaur_code_gen::records::operand_x_64::OperandX64;
    use luaur_code_gen::records::register_x_64::RegisterX64;

    let mut fixture = IrCallWrapperX64FixtureSystemV::new();
    let mut tmp = fixture.base.take_scoped(RegisterX64::r9);
    let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
        &mut *fixture.base.regs,
        &mut *fixture.base.build,
        !0u32,
    );

    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        RegisterX64::r12.into(),
        IrOp::ir_op(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        RegisterX64::r13.into(),
        IrOp::ir_op(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        RegisterX64::r14.into(),
        IrOp::ir_op(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::dword,
        OperandX64::from(2),
        IrOp::ir_op(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        OperandX64::from(1),
        IrOp::ir_op(),
    );

    let reg = call_wrap.suggest_next_argument_register(SizeX64::dword);
    fixture
        .base
        .build
        .mov(OperandX64::from(reg), OperandX64::from(10));
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::dword,
        OperandX64::from(reg),
        IrOp::ir_op(),
    );

    let func = OperandX64::from(tmp.release());
    call_wrap.call(&func);

    fixture.base.check_match(String::from(
        r#"
 mov         eax,Ah
 mov         rdi,r12
 mov         rsi,r13
 mov         rdx,r14
 mov         rcx,r9
 mov         r9d,eax
 mov         rax,rcx
 mov         ecx,2
 mov         r8,1
 call        rax
"#,
    ));
}
