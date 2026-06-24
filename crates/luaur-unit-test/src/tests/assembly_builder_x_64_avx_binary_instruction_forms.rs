#[cfg(test)]
#[test]
fn assembly_builder_x_64_avx_binary_instruction_forms() {
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
        |b| b.vaddpd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x29, 0x58, 0xc6],
    );
    check(
        |b| b.vaddpd(R::xmm8.into(), R::xmm10.into(), idx(xmmword, R::r9)),
        &[0xc4, 0x41, 0x29, 0x58, 0x01],
    );
    check(
        |b| b.vaddpd(R::ymm8.into(), R::ymm10.into(), R::ymm14.into()),
        &[0xc4, 0x41, 0x2d, 0x58, 0xc6],
    );
    check(
        |b| b.vaddpd(R::ymm8.into(), R::ymm10.into(), idx(ymmword, R::r9)),
        &[0xc4, 0x41, 0x2d, 0x58, 0x01],
    );
    check(
        |b| b.vaddps(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x28, 0x58, 0xc6],
    );
    check(
        |b| b.vaddps(R::xmm8.into(), R::xmm10.into(), idx(xmmword, R::r9)),
        &[0xc4, 0x41, 0x28, 0x58, 0x01],
    );
    check(
        |b| b.vaddsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0x58, 0xc6],
    );
    check(
        |b| b.vaddsd(R::xmm8.into(), R::xmm10.into(), idx(qword, R::r9)),
        &[0xc4, 0x41, 0x2b, 0x58, 0x01],
    );
    check(
        |b| b.vaddss(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2a, 0x58, 0xc6],
    );
    check(
        |b| b.vaddss(R::xmm8.into(), R::xmm10.into(), idx(dword, R::r9)),
        &[0xc4, 0x41, 0x2a, 0x58, 0x01],
    );

    check(
        |b| b.vaddps(R::xmm1.into(), R::xmm2.into(), R::xmm3.into()),
        &[0xc4, 0xe1, 0x68, 0x58, 0xcb],
    );
    check(
        |b| {
            b.vaddps(
                R::xmm9.into(),
                R::xmm12.into(),
                idx(xmmword, R::r9 + R::r14 * 2 + 0x1c),
            )
        },
        &[0xc4, 0x01, 0x18, 0x58, 0x4c, 0x71, 0x1c],
    );
    check(
        |b| b.vaddps(R::ymm1.into(), R::ymm2.into(), R::ymm3.into()),
        &[0xc4, 0xe1, 0x6c, 0x58, 0xcb],
    );
    check(
        |b| {
            b.vaddps(
                R::ymm9.into(),
                R::ymm12.into(),
                idx(ymmword, R::r9 + R::r14 * 2 + 0x1c),
            )
        },
        &[0xc4, 0x01, 0x1c, 0x58, 0x4c, 0x71, 0x1c],
    );

    // Coverage for other instructions that follow the same pattern
    check(
        |b| b.vsubsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0x5c, 0xc6],
    );
    check(
        |b| b.vmulsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0x59, 0xc6],
    );
    check(
        |b| b.vdivsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0x5e, 0xc6],
    );

    check(
        |b| b.vsubps(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x28, 0x5c, 0xc6],
    );
    check(
        |b| b.vmulps(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x28, 0x59, 0xc6],
    );
    check(
        |b| b.vdivps(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x28, 0x5e, 0xc6],
    );

    check(
        |b| b.vorpd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x29, 0x56, 0xc6],
    );
    check(
        |b| b.vxorpd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x29, 0x57, 0xc6],
    );
    check(
        |b| b.vorps(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x28, 0x56, 0xc6],
    );

    check(
        |b| b.vandpd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x29, 0x54, 0xc6],
    );
    check(
        |b| b.vandnpd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x29, 0x55, 0xc6],
    );

    check(
        |b| b.vmaxsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0x5f, 0xc6],
    );
    check(
        |b| b.vminsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0x5d, 0xc6],
    );

    check(
        |b| b.vmaxss(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2a, 0x5f, 0xc6],
    );
    check(
        |b| b.vminss(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2a, 0x5d, 0xc6],
    );

    check(
        |b| b.vmaxps(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x28, 0x5f, 0xc6],
    );
    check(
        |b| b.vminps(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x28, 0x5d, 0xc6],
    );

    check(
        |b| b.vcmpeqsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0xc2, 0xc6, 0x00],
    );
    check(
        |b| b.vcmpltsd(R::xmm8.into(), R::xmm10.into(), R::xmm14.into()),
        &[0xc4, 0x41, 0x2b, 0xc2, 0xc6, 0x01],
    );
}
