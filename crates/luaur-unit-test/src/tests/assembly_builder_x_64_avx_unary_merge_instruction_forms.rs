#[cfg(test)]
#[test]
fn assembly_builder_x_64_avx_unary_merge_instruction_forms() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{dword, qword, xmmword, ymmword, OperandX64};
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
        |b| b.vsqrtpd(R::xmm8.into(), R::xmm10.into()),
        &[0xc4, 0x41, 0x79, 0x51, 0xc2],
    );
    check(
        |b| b.vsqrtpd(R::xmm8.into(), idx(xmmword, R::r9)),
        &[0xc4, 0x41, 0x79, 0x51, 0x01],
    );
    check(
        |b| b.vsqrtpd(R::ymm8.into(), R::ymm10.into()),
        &[0xc4, 0x41, 0x7d, 0x51, 0xc2],
    );
    check(
        |b| b.vsqrtpd(R::ymm8.into(), idx(ymmword, R::r9)),
        &[0xc4, 0x41, 0x7d, 0x51, 0x01],
    );
    check(
        |b| b.vsqrtps(R::xmm8.into(), R::xmm10.into()),
        &[0xc4, 0x41, 0x78, 0x51, 0xc2],
    );
    check(
        |b| b.vsqrtps(R::xmm8.into(), idx(xmmword, R::r9)),
        &[0xc4, 0x41, 0x78, 0x51, 0x01],
    );
    check(
        |b| b.vsqrtsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0x51, 0xc6],
    );
    check(
        |b| b.vsqrtsd(R::xmm8.into(), R::xmm10.into(), idx(qword, R::r9)),
        &[0xc4, 0x41, 0x2b, 0x51, 0x01],
    );
    check(
        |b| b.vsqrtss(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2a, 0x51, 0xc6],
    );
    check(
        |b| b.vsqrtss(R::xmm8.into(), R::xmm10.into(), idx(dword, R::r9)),
        &[0xc4, 0x41, 0x2a, 0x51, 0x01],
    );

    // Coverage for other instructions that follow the same pattern
    check(
        |b| b.vucomisd(R::xmm1.into(), R::xmm4.into()),
        &[0xc4, 0xe1, 0x79, 0x2e, 0xcc],
    );
}
