#[cfg(test)]
#[test]
fn assembly_builder_a_64_fp_insert_extract() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    check(
        |b| b.ins_4_s_register_a_64_register_a_64_u8(RegisterA64::q29, RegisterA64::w17, 3),
        0x4E1C1E3D,
    );
    check(
        |b| b.ins_4_s_register_a_64_u8_register_a_64_u8(RegisterA64::q31, 0, RegisterA64::q29, 0),
        0x6E0407BF,
    );
    check(
        |b| b.dup_4s(RegisterA64::s29, RegisterA64::q31, 2),
        0x5E1407FD,
    );
    check(
        |b| b.dup_4s(RegisterA64::q29, RegisterA64::q30, 0),
        0x4E0407DD,
    );
    check(
        |b| b.umov_4s(RegisterA64::w1, RegisterA64::q30, 3),
        0x0E1C3FC1,
    );
    check(
        |b| b.umov_4s(RegisterA64::w13, RegisterA64::q1, 1),
        0x0E0C3C2D,
    );

    check(
        |b| b.bit(RegisterA64::q1, RegisterA64::q2, RegisterA64::q3),
        0x6EA31C41,
    );
    check(
        |b| b.bif(RegisterA64::q1, RegisterA64::q2, RegisterA64::q3),
        0x6EE31C41,
    );
}
