#[cfg(test)]
#[test]
fn assembly_builder_x_64_avx_ternary_instruction_forms() {
    use luaur_code_gen::enums::rounding_mode_x_64::RoundingModeX64;
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::operand_x_64::{xmmword, OperandX64};
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
        |b| {
            b.vroundsd(
                R::xmm7.into(),
                R::xmm12.into(),
                R::xmm3.into(),
                RoundingModeX64::RoundToNegativeInfinity,
            )
        },
        &[0xc4, 0xe3, 0x19, 0x0b, 0xfb, 0x09],
    );
    check(
        |b| {
            b.vroundsd(
                R::xmm8.into(),
                R::xmm13.into(),
                idx(xmmword, R::r13 + R::rdx),
                RoundingModeX64::RoundToPositiveInfinity,
            )
        },
        &[0xc4, 0x43, 0x11, 0x0b, 0x44, 0x15, 0x00, 0x0a],
    );
    check(
        |b| {
            b.vroundsd(
                R::xmm9.into(),
                R::xmm14.into(),
                idx(xmmword, R::rcx + R::r10),
                RoundingModeX64::RoundToZero,
            )
        },
        &[0xc4, 0x23, 0x09, 0x0b, 0x0c, 0x11, 0x0b],
    );

    check(
        |b| {
            b.vroundps(
                R::xmm1.into(),
                R::xmm3.into(),
                RoundingModeX64::RoundToNegativeInfinity,
            )
        },
        &[0xc4, 0xe3, 0x79, 0x08, 0xcb, 0x09],
    );
    check(
        |b| {
            b.vroundps(
                R::xmm12.into(),
                R::xmm14.into(),
                RoundingModeX64::RoundToNegativeInfinity,
            )
        },
        &[0xc4, 0x43, 0x79, 0x08, 0xe6, 0x09],
    );
    check(
        |b| {
            b.vroundps(
                R::xmm12.into(),
                idx(xmmword, R::rax + R::r13),
                RoundingModeX64::RoundToNegativeInfinity,
            )
        },
        &[0xc4, 0x23, 0x79, 0x08, 0x24, 0x28, 0x09],
    );

    check(
        |b| b.vblendvpd(R::xmm7, R::xmm12, idx(xmmword, R::rcx + R::r10), R::xmm5),
        &[0xc4, 0xa3, 0x19, 0x4b, 0x3c, 0x11, 0x50],
    );

    check(
        |b| b.vpshufps(R::xmm7, R::xmm12, idx(xmmword, R::rcx + R::r10), 0b11010100),
        &[0xc4, 0xa1, 0x18, 0xc6, 0x3c, 0x11, 0xd4],
    );
    check(
        |b| b.vpinsrd(R::xmm7, R::xmm12, idx(xmmword, R::rcx + R::r10), 2),
        &[0xc4, 0xa3, 0x19, 0x22, 0x3c, 0x11, 0x02],
    );

    check(
        |b| b.vpextrd(R::ecx, R::xmm5, 2),
        &[0xc4, 0xe3, 0x79, 0x16, 0xe9, 0x02],
    );
    check(
        |b| b.vpextrd(R::r10d, R::xmm9, 1),
        &[0xc4, 0x43, 0x79, 0x16, 0xca, 0x01],
    );

    check(
        |b| {
            b.vdpps(
                R::xmm7.into(),
                R::xmm12.into(),
                idx(xmmword, R::rcx + R::r10),
                2,
            )
        },
        &[0xc4, 0xa3, 0x19, 0x40, 0x3c, 0x11, 0x02],
    );
}
