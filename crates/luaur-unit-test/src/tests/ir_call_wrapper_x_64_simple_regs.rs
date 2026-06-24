//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/IrCallWrapperX64.test.cpp:56:ir_call_wrapper_x_64_simple_regs`
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
//!   - translates_to -> rust_item ir_call_wrapper_x_64_simple_regs

#[cfg(test)]
#[test]
fn ir_call_wrapper_x_64_simple_regs() {
    use crate::records::ir_call_wrapper_x_64_fixture::IrCallWrapperX64Fixture;
    use luaur_code_gen::enums::size_x_64::SizeX64;
    use luaur_code_gen::records::ir_data::k_invalid_inst_idx;
    use luaur_code_gen::records::operand_x_64::qword;
    use luaur_code_gen::records::register_x_64::RegisterX64;
    use luaur_code_gen::records::scoped_reg_x_64::ScopedRegX64;

    let mut fixture = IrCallWrapperX64Fixture::windows();

    let tmp1_reg = fixture.regs.take_reg(RegisterX64::rax, k_invalid_inst_idx);
    let mut tmp1 = ScopedRegX64 {
        owner: &mut *fixture.regs,
        reg: tmp1_reg,
    };
    let tmp2_reg = fixture.regs.take_reg(fixture.r_arg2, k_invalid_inst_idx);
    let mut tmp2 = ScopedRegX64 {
        owner: &mut *fixture.regs,
        reg: tmp2_reg,
    };

    fixture
        .call_wrap
        .add_argument_size_x_64_scoped_reg_x_64(SizeX64::qword, &mut tmp1);
    fixture
        .call_wrap
        .add_argument_size_x_64_scoped_reg_x_64(SizeX64::qword, &mut tmp2);
    fixture
        .call_wrap
        .call(&qword.operator_bracket(RegisterX64::r12.into()));

    fixture.check_match(String::from(
        r#"
 mov         rcx,rax
 call        qword ptr [r12]
"#,
    ));
}
