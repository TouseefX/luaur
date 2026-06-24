#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_absolute_jumps() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{qword, OperandX64};
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, address: impl Into<OperandX64>) -> OperandX64 {
        prefix.operator_bracket(address.into())
    }

    check(|b| b.jmp_operand_x_64(R::rax.into()), &[0xff, 0xe0]);
    check(|b| b.jmp_operand_x_64(R::r14.into()), &[0x41, 0xff, 0xe6]);
    check(
        |b| b.jmp_operand_x_64(idx(qword, R::r14 + R::rdx * 4)),
        &[0x41, 0xff, 0x24, 0x96],
    );
    check(|b| b.call_operand_x_64(R::rax.into()), &[0xff, 0xd0]);
    check(|b| b.call_operand_x_64(R::r14.into()), &[0x41, 0xff, 0xd6]);
    check(
        |b| b.call_operand_x_64(idx(qword, R::r14 + R::rdx * 4)),
        &[0x41, 0xff, 0x14, 0x96],
    );
}
