#[cfg(test)]
#[test]
fn assembly_builder_a_64_binary_imm() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    // instructions
    check(
        |b| b.and_register_a_64_register_a_64_u32(R::w1.into(), R::w2.into(), 1),
        0x12000041,
    );
    check(
        |b| b.orr_register_a_64_register_a_64_u32(R::w1.into(), R::w2.into(), 1),
        0x32000041,
    );
    check(
        |b| b.eor_register_a_64_register_a_64_u32(R::w1.into(), R::w2.into(), 1),
        0x52000041,
    );
    check(|b| b.tst_register_a_64_u32(R::w1.into(), 1), 0x7200003f);

    // various mask forms
    check(
        |b| b.and_register_a_64_register_a_64_u32(R::w0.into(), R::w0.into(), 1),
        0x12000000,
    );
    check(
        |b| b.and_register_a_64_register_a_64_u32(R::w0.into(), R::w0.into(), 3),
        0x12000400,
    );
    check(
        |b| b.and_register_a_64_register_a_64_u32(R::w0.into(), R::w0.into(), 7),
        0x12000800,
    );
    check(
        |b| b.and_register_a_64_register_a_64_u32(R::w0.into(), R::w0.into(), 2147483647),
        0x12007800,
    );
    check(
        |b| b.and_register_a_64_register_a_64_u32(R::w0.into(), R::w0.into(), 6),
        0x121F0400,
    );
    check(
        |b| b.and_register_a_64_register_a_64_u32(R::w0.into(), R::w0.into(), 12),
        0x121E0400,
    );
    check(
        |b| b.and_register_a_64_register_a_64_u32(R::w0.into(), R::w0.into(), 2147483648),
        0x12010000,
    );

    // shifts
    check(
        |b| b.lsl_register_a_64_register_a_64_u8(R::w1.into(), R::w2.into(), 1),
        0x531F7841,
    );
    check(
        |b| b.lsl_register_a_64_register_a_64_u8(R::x1.into(), R::x2.into(), 1),
        0xD37FF841,
    );
    check(
        |b| b.lsr_register_a_64_register_a_64_u8(R::w1.into(), R::w2.into(), 1),
        0x53017C41,
    );
    check(
        |b| b.lsr_register_a_64_register_a_64_u8(R::x1.into(), R::x2.into(), 1),
        0xD341FC41,
    );
    check(
        |b| b.asr_register_a_64_register_a_64_u8(R::w1.into(), R::w2.into(), 1),
        0x13017C41,
    );
    check(
        |b| b.asr_register_a_64_register_a_64_u8(R::x1.into(), R::x2.into(), 1),
        0x9341FC41,
    );
    check(
        |b| b.ror_register_a_64_register_a_64_u8(R::w1.into(), R::w2.into(), 1),
        0x13820441,
    );
    check(
        |b| b.ror_register_a_64_register_a_64_u8(R::x1.into(), R::x2.into(), 1),
        0x93C20441,
    );
}
