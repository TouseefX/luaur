#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_mov() {
    use luaur_code_gen::functions::word_reg::word_reg;
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{byte, dword, qword, word, OperandX64};
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    // C++ `AssemblyBuilderX64Fixture::check`: a FRESH builder per case, run one
    // instruction, finalize, assert `build.code` matches the expected encoding.
    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }
    // `size[reg]` addressing: `operator[]` stamps the size prefix onto the address.
    fn mem(prefix: OperandX64, reg: R) -> OperandX64 {
        prefix.operator_bracket(OperandX64::from(reg))
    }

    check(
        |b| b.mov(R::rcx.into(), 1i32.into()),
        &[0x48, 0xb9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.mov64(R::rcx, 0x1234567812345678i64),
        &[0x48, 0xb9, 0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12],
    );
    check(
        |b| b.mov(R::ecx.into(), 2i32.into()),
        &[0xb9, 0x02, 0x00, 0x00, 0x00],
    );
    check(|b| b.mov(R::cl.into(), 2i32.into()), &[0xb1, 0x02]);
    check(|b| b.mov(R::sil.into(), 2i32.into()), &[0x40, 0xb6, 0x02]);
    check(|b| b.mov(R::r9b.into(), 2i32.into()), &[0x41, 0xb1, 0x02]);
    check(
        |b| b.mov(R::rcx.into(), mem(qword, R::rdi)),
        &[0x48, 0x8b, 0x0f],
    );
    check(
        |b| b.mov(mem(dword, R::rax), 0xabcdi32.into()),
        &[0xc7, 0x00, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.mov(R::r13.into(), 1i32.into()),
        &[0x49, 0xbd, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.mov64(R::r13, 0x1234567812345678i64),
        &[0x49, 0xbd, 0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12],
    );
    check(
        |b| b.mov(R::r13d.into(), 2i32.into()),
        &[0x41, 0xbd, 0x02, 0x00, 0x00, 0x00],
    );
    check(
        |b| b.mov(R::r13.into(), mem(qword, R::r12)),
        &[0x4d, 0x8b, 0x2c, 0x24],
    );
    check(
        |b| b.mov(mem(dword, R::r13), 0xabcdi32.into()),
        &[0x41, 0xc7, 0x45, 0x00, 0xcd, 0xab, 0x00, 0x00],
    );
    check(
        |b| b.mov(mem(qword, R::rdx), R::r9.into()),
        &[0x4c, 0x89, 0x0a],
    );
    check(
        |b| b.mov(mem(byte, R::rsi), 0x3i32.into()),
        &[0xc6, 0x06, 0x03],
    );
    check(|b| b.mov(mem(byte, R::rsi), R::al.into()), &[0x88, 0x06]);
    check(
        |b| b.mov(mem(byte, R::rsi), R::dil.into()),
        &[0x40, 0x88, 0x3e],
    );
    check(
        |b| b.mov(mem(byte, R::rsi), R::r10b.into()),
        &[0x44, 0x88, 0x16],
    );
    check(
        |b| b.mov(word_reg(R::ebx).into(), 0x3a3di32.into()),
        &[0x66, 0xbb, 0x3d, 0x3a],
    );
    check(
        |b| b.mov(mem(word, R::rsi), 0x3a3di32.into()),
        &[0x66, 0xc7, 0x06, 0x3d, 0x3a],
    );
    check(
        |b| b.mov(mem(word, R::rsi), word_reg(R::eax).into()),
        &[0x66, 0x89, 0x06],
    );
    check(
        |b| b.mov(mem(word, R::rsi), word_reg(R::edi).into()),
        &[0x66, 0x89, 0x3e],
    );
    check(
        |b| b.mov(mem(word, R::rsi), word_reg(R::r10).into()),
        &[0x66, 0x44, 0x89, 0x16],
    );
}
