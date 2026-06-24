#[cfg(test)]
#[test]
fn assembly_builder_x_64_alignment_forms() {
    use luaur_code_gen::enums::alignment_data_x_64::AlignmentDataX64;
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(
        |b| {
            b.ret();
            b.align(8, AlignmentDataX64::Nop);
        },
        &[0xc3, 0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00],
    );

    check(
        |b| {
            b.ret();
            b.align(32, AlignmentDataX64::Nop);
        },
        &[
            0xc3, 0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x1f, 0x84,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x0f, 0x1f, 0x40, 0x00,
        ],
    );

    check(
        |b| {
            b.ret();
            b.align(8, AlignmentDataX64::Int3);
        },
        &[0xc3, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc],
    );

    check(
        |b| {
            b.ret();
            b.align(8, AlignmentDataX64::Ud2);
        },
        &[0xc3, 0x0f, 0x0b, 0x0f, 0x0b, 0x0f, 0x0b, 0xcc],
    );
}
