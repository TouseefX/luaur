//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:256:ir_call_wrapper_x_64_hard_interference_both`
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
//!   - translates_to -> rust_item ir_call_wrapper_x_64_hard_interference_both

#[cfg(test)]
#[test]
fn ir_call_wrapper_x_64_hard_interference_both() {
    use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
    use luaur_code_gen::enums::size_x_64::SizeX64;
    use luaur_code_gen::records::operand_x_64::qword;
    use luaur_code_gen::records::register_x_64::RegisterX64;

    let mut fixture = IrCallWrapperX64Fixture::windows();
    let mut int1 = fixture.take_scoped(fixture.r_arg2);
    let mut int2 = fixture.take_scoped(fixture.r_arg1);
    let mut fp1 = fixture.take_scoped(RegisterX64::xmm3);
    let mut fp2 = fixture.take_scoped(RegisterX64::xmm2);

    fixture.add_scoped(SizeX64::qword, &mut int1);
    fixture.add_scoped(SizeX64::qword, &mut int2);
    fixture.add_scoped(SizeX64::xmmword, &mut fp1);
    fixture.add_scoped(SizeX64::xmmword, &mut fp2);
    fixture.call(qword.operator_bracket(RegisterX64::r12.into()));

    fixture.check_match(String::from(
        r#"
 mov         rax,rdx
 mov         rdx,rcx
 mov         rcx,rax
 vmovsd      xmm0,xmm3,xmm3
 vmovsd      xmm3,xmm2,xmm2
 vmovsd      xmm2,xmm0,xmm0
 call        qword ptr [r12]
"#,
    ));
}
