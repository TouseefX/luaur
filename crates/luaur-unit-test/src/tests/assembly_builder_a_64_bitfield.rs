#[cfg(test)]
#[test]
fn assembly_builder_a_64_bitfield() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    check(
        |b| b.ubfiz(RegisterA64::x1, RegisterA64::x2, 37, 5),
        0xD35B1041,
    );
    check(
        |b| b.ubfx(RegisterA64::x1, RegisterA64::x2, 37, 5),
        0xD365A441,
    );
    check(
        |b| b.sbfiz(RegisterA64::x1, RegisterA64::x2, 37, 5),
        0x935B1041,
    );
    check(
        |b| b.sbfx_register_a_64_register_a_64_u8_u8(RegisterA64::x1, RegisterA64::x2, 37, 5),
        0x9365A441,
    );

    check(
        |b| b.ubfiz(RegisterA64::w1, RegisterA64::w2, 17, 5),
        0x530F1041,
    );
    check(
        |b| b.ubfx(RegisterA64::w1, RegisterA64::w2, 17, 5),
        0x53115441,
    );
    check(
        |b| b.sbfiz(RegisterA64::w1, RegisterA64::w2, 17, 5),
        0x130F1041,
    );
    check(
        |b| b.sbfx_register_a_64_register_a_64_u8_u8(RegisterA64::w1, RegisterA64::w2, 17, 5),
        0x13115441,
    );
}
