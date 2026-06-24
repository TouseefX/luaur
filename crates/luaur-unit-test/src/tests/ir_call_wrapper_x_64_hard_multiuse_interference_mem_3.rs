//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:332:ir_call_wrapper_x_64_hard_multiuse_interference_mem_3`
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
//!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_multiuse_interference_mem_3

#[cfg(test)]
#[test]
fn ir_call_wrapper_x_64_hard_multiuse_interference_mem3() {
    use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
    use luaur_code_gen::enums::size_x_64::SizeX64;
    use luaur_code_gen::records::operand_x_64::qword;
    use luaur_code_gen::records::register_x_64::RegisterX64;

    let mut fixture = IrCallWrapperX64Fixture::windows();
    let mut tmp1 = fixture.take_scoped(fixture.r_arg3);
    let mut tmp2 = fixture.take_scoped(fixture.r_arg2);
    let mut tmp3 = fixture.take_scoped(fixture.r_arg1);

    fixture.add_arg(
        SizeX64::qword,
        qword.operator_bracket(tmp1.reg + tmp2.reg + 8),
    );
    fixture.add_arg(
        SizeX64::qword,
        qword.operator_bracket(tmp2.reg + tmp3.reg + 16),
    );
    fixture.add_arg(
        SizeX64::qword,
        qword.operator_bracket(tmp3.reg + tmp1.reg + 16),
    );
    tmp1.release();
    tmp2.release();
    tmp3.release();
    fixture.call(qword.operator_bracket(RegisterX64::r12.into()));

    fixture.check_match(String::from(
        r#"
 mov         rax,r8
 mov         r8,qword ptr [rcx+rax+010h]
 mov         rbx,rdx
 mov         rdx,qword ptr [rbx+rcx+010h]
 mov         rcx,qword ptr [rax+rbx+8]
 call        qword ptr [r12]
"#,
    ));
}
