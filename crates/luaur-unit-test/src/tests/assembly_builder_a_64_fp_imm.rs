#[cfg(test)]
#[test]
fn assembly_builder_a_64_fp_imm() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(build: &mut AssemblyBuilderA64, expected: u32) {
        build.finalize();
        assert_eq!(build.code[0], expected, "instruction byte mismatch");
    }

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::d0, 0.0);
    check(&mut build, 0x2F00E400);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::d0, 0.125);
    check(&mut build, 0x1E681000);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::d0, -0.125);
    check(&mut build, 0x1E781000);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::d0, 1.9375);
    check(&mut build, 0x1E6FF000);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::q0, 0.0);
    check(&mut build, 0x4F000400);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::q0, 0.125);
    check(&mut build, 0x4F02F400);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::q0, -0.125);
    check(&mut build, 0x4F06F400);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.fmov_register_a_64_f64(RegisterA64::q0, 1.9375);
    check(&mut build, 0x4F03F7E0);

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    assert!(!build.is_fmov_supported_fp_64(-0.0));

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    assert!(!build.is_fmov_supported_fp_64(0.12389));
}
