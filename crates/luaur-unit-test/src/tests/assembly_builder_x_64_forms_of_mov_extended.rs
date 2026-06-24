#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_mov_extended() {
    use luaur_code_gen::functions::word_reg::word_reg;
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{byte, word, OperandX64};
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    fn mem(prefix: OperandX64, reg: R) -> OperandX64 {
        prefix.operator_bracket(OperandX64::from(reg))
    }

    check(|b| b.movsx(R::eax, mem(byte, R::rcx)), &[0x0f, 0xbe, 0x01]);
    check(
        |b| b.movsx(R::r12, mem(byte, R::r10)),
        &[0x4d, 0x0f, 0xbe, 0x22],
    );
    check(
        |b| b.movsx(R::ebx, mem(word, R::r11)),
        &[0x41, 0x0f, 0xbf, 0x1b],
    );
    check(
        |b| b.movsx(R::rdx, mem(word, R::rcx)),
        &[0x48, 0x0f, 0xbf, 0x11],
    );
    check(|b| b.movsx(R::edx, R::cl.into()), &[0x0f, 0xbe, 0xd1]);
    check(
        |b| b.movsx(R::edx, R::r12b.into()),
        &[0x41, 0x0f, 0xbe, 0xd4],
    );
    check(
        |b| b.movsx(R::edx, word_reg(R::ecx).into()),
        &[0x0f, 0xbf, 0xd1],
    );
    check(
        |b| b.movsx(R::edx, word_reg(R::r12d).into()),
        &[0x41, 0x0f, 0xbf, 0xd4],
    );
    check(|b| b.movzx(R::eax, mem(byte, R::rcx)), &[0x0f, 0xb6, 0x01]);
    check(
        |b| b.movzx(R::r12, mem(byte, R::r10)),
        &[0x4d, 0x0f, 0xb6, 0x22],
    );
    check(
        |b| b.movzx(R::ebx, mem(word, R::r11)),
        &[0x41, 0x0f, 0xb7, 0x1b],
    );
    check(
        |b| b.movzx(R::rdx, mem(word, R::rcx)),
        &[0x48, 0x0f, 0xb7, 0x11],
    );
    check(|b| b.movzx(R::edx, R::cl.into()), &[0x0f, 0xb6, 0xd1]);
    check(
        |b| b.movzx(R::edx, R::r12b.into()),
        &[0x41, 0x0f, 0xb6, 0xd4],
    );
    check(
        |b| b.movzx(R::edx, word_reg(R::ecx).into()),
        &[0x0f, 0xb7, 0xd1],
    );
    check(
        |b| b.movzx(R::edx, word_reg(R::r12d).into()),
        &[0x41, 0x0f, 0xb7, 0xd4],
    );
}
