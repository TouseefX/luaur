#[cfg(test)]
#[test]
fn assembly_builder_x_64_avx_move_instruction_forms() {
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
        |b| b.vmovsd_operand_x_64_operand_x_64(idx(qword, R::r9), R::xmm10.into()),
        &[0xc4, 0x41, 0x7b, 0x11, 0x11],
    );
    check(
        |b| b.vmovsd_operand_x_64_operand_x_64(R::xmm8.into(), idx(qword, R::r9)),
        &[0xc4, 0x41, 0x7b, 0x10, 0x01],
    );
    check(
        |b| {
            b.vmovsd_operand_x_64_operand_x_64_operand_x_64(
                R::xmm8.into(),
                R::xmm10.into(),
                R::xmm14.into(),
            )
        },
        &[0xc4, 0x41, 0x2b, 0x10, 0xc6],
    );
    check(
        |b| b.vmovss_operand_x_64_operand_x_64(idx(dword, R::r9), R::xmm10.into()),
        &[0xc4, 0x41, 0x7a, 0x11, 0x11],
    );
    check(
        |b| b.vmovss_operand_x_64_operand_x_64(R::xmm8.into(), idx(dword, R::r9)),
        &[0xc4, 0x41, 0x7a, 0x10, 0x01],
    );
    check(
        |b| {
            b.vmovss_operand_x_64_operand_x_64_operand_x_64(
                R::xmm8.into(),
                R::xmm10.into(),
                R::xmm14.into(),
            )
        },
        &[0xc4, 0x41, 0x2a, 0x10, 0xc6],
    );
    check(
        |b| b.vmovapd(R::xmm8.into(), idx(xmmword, R::r9)),
        &[0xc4, 0x41, 0x79, 0x28, 0x01],
    );
    check(
        |b| b.vmovapd(idx(xmmword, R::r9), R::xmm10.into()),
        &[0xc4, 0x41, 0x79, 0x29, 0x11],
    );
    check(
        |b| b.vmovapd(R::ymm8.into(), idx(ymmword, R::r9)),
        &[0xc4, 0x41, 0x7d, 0x28, 0x01],
    );
    check(
        |b| b.vmovaps(R::xmm8.into(), idx(xmmword, R::r9)),
        &[0xc4, 0x41, 0x78, 0x28, 0x01],
    );
    check(
        |b| b.vmovaps(idx(xmmword, R::r9), R::xmm10.into()),
        &[0xc4, 0x41, 0x78, 0x29, 0x11],
    );
    check(
        |b| b.vmovaps(R::ymm8.into(), idx(ymmword, R::r9)),
        &[0xc4, 0x41, 0x7c, 0x28, 0x01],
    );
    check(
        |b| b.vmovupd(R::xmm8.into(), idx(xmmword, R::r9)),
        &[0xc4, 0x41, 0x79, 0x10, 0x01],
    );
    check(
        |b| b.vmovupd(idx(xmmword, R::r9), R::xmm10.into()),
        &[0xc4, 0x41, 0x79, 0x11, 0x11],
    );
    check(
        |b| b.vmovupd(R::ymm8.into(), idx(ymmword, R::r9)),
        &[0xc4, 0x41, 0x7d, 0x10, 0x01],
    );
    check(
        |b| b.vmovups(R::xmm8.into(), idx(xmmword, R::r9)),
        &[0xc4, 0x41, 0x78, 0x10, 0x01],
    );
    check(
        |b| b.vmovups(idx(xmmword, R::r9), R::xmm10.into()),
        &[0xc4, 0x41, 0x78, 0x11, 0x11],
    );
    check(
        |b| b.vmovups(R::ymm8.into(), idx(ymmword, R::r9)),
        &[0xc4, 0x41, 0x7c, 0x10, 0x01],
    );
    check(
        |b| b.vmovq(R::xmm1.into(), R::rbx.into()),
        &[0xc4, 0xe1, 0xf9, 0x6e, 0xcb],
    );
    check(
        |b| b.vmovq(R::rbx.into(), R::xmm1.into()),
        &[0xc4, 0xe1, 0xf9, 0x7e, 0xcb],
    );
    check(
        |b| b.vmovq(R::xmm1.into(), idx(qword, R::r9)),
        &[0xc4, 0xc1, 0xf9, 0x6e, 0x09],
    );
    check(
        |b| b.vmovq(idx(qword, R::r9), R::xmm1.into()),
        &[0xc4, 0xc1, 0xf9, 0x7e, 0x09],
    );
}
