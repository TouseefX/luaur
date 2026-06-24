#[cfg(test)]
#[test]
fn assembly_builder_a_64_fp_compare() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), expected: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code.len(), 1, "expected exactly one instruction");
        assert_eq!(build.code[0], expected, "instruction word mismatch");
    }

    check(|b| b.fcmp(RegisterA64::d0, RegisterA64::d1), 0x1E612000);
    check(|b| b.fcmpz(RegisterA64::d1), 0x1E602028);

    check(
        |b| b.fcmeq_4s(RegisterA64::q1, RegisterA64::q2, RegisterA64::q3),
        0x4E23E441,
    );
    check(
        |b| b.fcmgt_4s(RegisterA64::q1, RegisterA64::q2, RegisterA64::q3),
        0x6EA3E441,
    );
}
