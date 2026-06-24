//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:116:ir_call_wrapper_x_64_simple_stack_args`
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
//!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
//!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - translates_to -> rust_item ir_call_wrapper_x_64_simple_stack_args

#[cfg(test)]
#[test]
fn ir_call_wrapper_x_64_simple_stack_args() {
    use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
    use luaur_code_gen::enums::size_x_64::SizeX64;
    use luaur_code_gen::records::operand_x_64::qword;
    use luaur_code_gen::records::register_x_64::RegisterX64;

    let mut fixture = IrCallWrapperX64Fixture::windows();
    let mut tmp = fixture.take_scoped(RegisterX64::rax);

    fixture.add_scoped(SizeX64::qword, &mut tmp);
    fixture.add_arg(
        SizeX64::qword,
        qword.operator_bracket(RegisterX64::r14 + 16),
    );
    fixture.add_arg(
        SizeX64::qword,
        qword.operator_bracket(RegisterX64::r14 + 32),
    );
    fixture.add_arg(
        SizeX64::qword,
        qword.operator_bracket(RegisterX64::r14 + 48),
    );
    fixture.add_arg(SizeX64::dword, 1);
    fixture.add_arg(
        SizeX64::qword,
        qword.operator_bracket(RegisterX64::r13.into()),
    );
    fixture.call(qword.operator_bracket(RegisterX64::r12.into()));

    fixture.check_match(String::from(
        r#"
 mov         rdx,qword ptr [r13]
 mov         qword ptr [rsp+028h],rdx
 mov         rcx,rax
 mov         rdx,qword ptr [r14+010h]
 mov         r8,qword ptr [r14+020h]
 mov         r9,qword ptr [r14+030h]
 mov         dword ptr [rsp+020h],1
 call        qword ptr [r12]
"#,
    ));
}
