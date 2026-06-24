#[cfg(test)]
#[test]
fn assembly_builder_a_64_binary() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    // reg, reg
    check(
        |b| {
            b.add_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                0,
            )
        },
        0x8B020020,
    );
    check(
        |b| {
            b.add_register_a_64_register_a_64_register_a_64_i32(
                R::w0.into(),
                R::w1.into(),
                R::w2.into(),
                0,
            )
        },
        0x0B020020,
    );
    check(
        |b| {
            b.add_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                7,
            )
        },
        0x8B021C20,
    );
    check(
        |b| {
            b.add_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                -7,
            )
        },
        0x8B421C20,
    );
    check(
        |b| {
            b.sub_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                0,
            )
        },
        0xCB020020,
    );
    check(
        |b| {
            b.and_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                0,
            )
        },
        0x8A020020,
    );
    check(
        |b| {
            b.and_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                7,
            )
        },
        0x8A021C20,
    );
    check(
        |b| {
            b.and_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                -7,
            )
        },
        0x8A421C20,
    );
    check(
        |b| b.bic(R::x0.into(), R::x1.into(), R::x2.into(), 0),
        0x8A220020,
    );
    check(
        |b| {
            b.orr_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                0,
            )
        },
        0xAA020020,
    );
    check(
        |b| {
            b.eor_register_a_64_register_a_64_register_a_64_i32(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
                0,
            )
        },
        0xCA020020,
    );
    check(
        |b| {
            b.lsl_register_a_64_register_a_64_register_a_64(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
            )
        },
        0x9AC22020,
    );
    check(
        |b| {
            b.lsl_register_a_64_register_a_64_register_a_64(
                R::w0.into(),
                R::w1.into(),
                R::w2.into(),
            )
        },
        0x1AC22020,
    );
    check(
        |b| {
            b.lsr_register_a_64_register_a_64_register_a_64(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
            )
        },
        0x9AC22420,
    );
    check(
        |b| {
            b.asr_register_a_64_register_a_64_register_a_64(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
            )
        },
        0x9AC22820,
    );
    check(
        |b| {
            b.ror_register_a_64_register_a_64_register_a_64(
                R::x0.into(),
                R::x1.into(),
                R::x2.into(),
            )
        },
        0x9AC22C20,
    );
    check(
        |b| b.cmp_register_a_64_register_a_64(R::x0.into(), R::x1.into()),
        0xEB01001F,
    );
    check(
        |b| b.tst_register_a_64_register_a_64_i32(R::x0.into(), R::x1.into(), 0),
        0xEA01001F,
    );

    // reg, imm
    check(
        |b| b.add_register_a_64_register_a_64_u16(R::x3.into(), R::x7.into(), 78),
        0x910138E3,
    );
    check(
        |b| b.add_register_a_64_register_a_64_u16(R::w3.into(), R::w7.into(), 78),
        0x110138E3,
    );
    check(
        |b| b.sub_register_a_64_register_a_64_u16(R::w3.into(), R::w7.into(), 78),
        0x510138E3,
    );
    check(|b| b.cmp_register_a_64_u16(R::w0.into(), 42), 0x7100A81F);
}
