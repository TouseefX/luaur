#[cfg(test)]
#[test]
fn assembly_builder_a_64_fp_basic() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(build: &mut AssemblyBuilderA64, expected: u32) {
        build.finalize();
        assert_eq!(build.code[0], expected, "instruction byte mismatch");
    }

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    let d0 = RegisterA64::d0;
    let d1 = RegisterA64::d1;
    let x1 = RegisterA64::x1;
    let x3 = RegisterA64::x3;
    let d2 = RegisterA64::d2;

    build.fmov_register_a_64_register_a_64(d0, d1);
    check(&mut build, 0x1E604020);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_register_a_64(d0, x1);
    check(&mut build, 0x9E670020);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_register_a_64(x3, d2);
    check(&mut build, 0x9E660043);
}
