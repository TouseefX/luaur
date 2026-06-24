#[cfg(test)]
#[test]
fn assembly_builder_x_64_avx_conversion_instruction_forms() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{dword, qword, xmmword, OperandX64};
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

    check(
        |b| b.vcvttsd2si(R::ecx.into(), R::xmm0.into()),
        &[0xc4, 0xe1, 0x7b, 0x2c, 0xc8],
    );
    check(
        |b| b.vcvttsd2si(R::r9d.into(), idx(xmmword, R::rcx + R::rdx)),
        &[0xc4, 0x61, 0x7b, 0x2c, 0x0c, 0x11],
    );
    check(
        |b| b.vcvttsd2si(R::rdx.into(), R::xmm0.into()),
        &[0xc4, 0xe1, 0xfb, 0x2c, 0xd0],
    );
    check(
        |b| b.vcvttsd2si(R::r13.into(), idx(xmmword, R::rcx + R::rdx)),
        &[0xc4, 0x61, 0xfb, 0x2c, 0x2c, 0x11],
    );
    check(
        |b| b.vcvtsi2sd(R::xmm5.into(), R::xmm10.into(), R::ecx.into()),
        &[0xc4, 0xe1, 0x2b, 0x2a, 0xe9],
    );
    check(
        |b| b.vcvtsi2sd(R::xmm6.into(), R::xmm11.into(), idx(dword, R::rcx + R::rdx)),
        &[0xc4, 0xe1, 0x23, 0x2a, 0x34, 0x11],
    );
    check(
        |b| b.vcvtsi2sd(R::xmm5.into(), R::xmm10.into(), R::r13.into()),
        &[0xc4, 0xc1, 0xab, 0x2a, 0xed],
    );
    check(
        |b| b.vcvtsi2sd(R::xmm6.into(), R::xmm11.into(), idx(qword, R::rcx + R::rdx)),
        &[0xc4, 0xe1, 0xa3, 0x2a, 0x34, 0x11],
    );
    check(
        |b| b.vcvtsd2ss(R::xmm5.into(), R::xmm10.into(), R::xmm11.into()),
        &[0xc4, 0xc1, 0x2b, 0x5a, 0xeb],
    );
    check(
        |b| b.vcvtsd2ss(R::xmm6.into(), R::xmm11.into(), idx(qword, R::rcx + R::rdx)),
        &[0xc4, 0xe1, 0xa3, 0x5a, 0x34, 0x11],
    );
    check(
        |b| b.vcvtss2sd(R::xmm3.into(), R::xmm8.into(), R::xmm12.into()),
        &[0xc4, 0xc1, 0x3a, 0x5a, 0xdc],
    );
    check(
        |b| b.vcvtss2sd(R::xmm4.into(), R::xmm9.into(), idx(dword, R::rcx + R::rsi)),
        &[0xc4, 0xe1, 0x32, 0x5a, 0x24, 0x31],
    );
}
