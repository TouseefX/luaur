//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:493:ir_call_wrapper_x_64_address_in_stack_arguments`
//! Source: `tests/IrCallWrapperX64.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/IrCallWrapperX64.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/IrCallWrapperX64.h
//!   - includes -> source_file CodeGen/include/Luau/IrRegAllocX64.h
//! - incoming:
//!   - declares <- source_file tests/IrCallWrapperX64.test.cpp
//! - outgoing:
//!   - type_ref -> enum SizeX64 (CodeGen/include/Luau/RegisterX64.h)
//!   - calls -> method IrCallWrapperX64Fixture::checkMatch (tests/IrCallWrapperX64.test.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - translates_to -> rust_item ir_call_wrapper_x_64_address_in_stack_arguments

#[cfg(test)]
#[test]
fn ir_call_wrapper_x_64_address_in_stack_arguments() {
    use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
    use luaur_code_gen::enums::size_x_64::SizeX64;
    use luaur_code_gen::records::operand_x_64::{addr, qword};
    use luaur_code_gen::records::register_x_64::RegisterX64;

    let mut fixture = IrCallWrapperX64Fixture::windows();
    fixture.add_arg(SizeX64::dword, 1);
    fixture.add_arg(SizeX64::dword, 2);
    fixture.add_arg(SizeX64::dword, 3);
    fixture.add_arg(SizeX64::dword, 4);
    fixture.add_arg(SizeX64::qword, addr.operator_bracket(RegisterX64::r12 + 16));
    fixture.call(qword.operator_bracket(RegisterX64::r14.into()));

    fixture.check_match(String::from(
        r#"
 lea         rax,[r12+010h]
 mov         qword ptr [rsp+020h],rax
 mov         ecx,1
 mov         edx,2
 mov         r8d,3
 mov         r9d,4
 call        qword ptr [r14]
"#,
    ));
}
