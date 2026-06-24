#[cfg(test)]
#[test]
fn assembly_builder_x_64_base_unary_instruction_forms() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{byte, qword, OperandX64};
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn idx(prefix: OperandX64, addr: impl Into<OperandX64>) -> OperandX64 {
        prefix.operator_bracket(addr.into())
    }

    check(|b| b.div(R::rcx.into()), &[0x48, 0xf7, 0xf1]);
    check(|b| b.idiv(idx(qword, R::rax)), &[0x48, 0xf7, 0x38]);
    check(
        |b| b.mul(idx(qword, R::rax + R::rbx)),
        &[0x48, 0xf7, 0x24, 0x18],
    );
    check(|b| b.imul_operand_x_64(R::r9.into()), &[0x49, 0xf7, 0xe9]);
    check(|b| b.neg(R::r9.into()), &[0x49, 0xf7, 0xd9]);
    check(|b| b.not_(R::r12.into()), &[0x49, 0xf7, 0xd4]);
    check(|b| b.inc(R::r12.into()), &[0x49, 0xff, 0xc4]);
    check(|b| b.dec(R::ecx.into()), &[0xff, 0xc9]);
    check(|b| b.dec(idx(byte, R::rdx)), &[0xfe, 0x0a]);
}
