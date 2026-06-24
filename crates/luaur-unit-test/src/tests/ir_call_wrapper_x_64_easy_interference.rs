//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:158:ir_call_wrapper_x_64_easy_interference`
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
//!   - translates_to -> rust_item ir_call_wrapper_x_64_easy_interference

#[cfg(test)]
#[test]
fn ir_call_wrapper_x_64_easy_interference() {
    use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
    use luaur_code_gen::enums::size_x_64::SizeX64;
    use luaur_code_gen::records::operand_x_64::qword;
    use luaur_code_gen::records::register_x_64::RegisterX64;

    let mut fixture = IrCallWrapperX64Fixture::windows();
    let mut tmp1 = fixture.take_scoped(RegisterX64::rdi);
    let mut tmp2 = fixture.take_scoped(RegisterX64::rsi);
    let mut tmp3 = fixture.take_scoped(fixture.r_arg2);
    let mut tmp4 = fixture.take_scoped(fixture.r_arg1);

    fixture.add_scoped(SizeX64::qword, &mut tmp1);
    fixture.add_scoped(SizeX64::qword, &mut tmp2);
    fixture.add_scoped(SizeX64::qword, &mut tmp3);
    fixture.add_scoped(SizeX64::qword, &mut tmp4);
    fixture.call(qword.operator_bracket(RegisterX64::r12.into()));

    fixture.check_match(String::from(
        r#"
 mov         r8,rdx
 mov         rdx,rsi
 mov         r9,rcx
 mov         rcx,rdi
 call        qword ptr [r12]
"#,
    ));
}
