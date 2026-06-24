#[cfg(test)]
#[test]
fn assembly_builder_a_64_moves() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_register_a_64(RegisterA64::x0, RegisterA64::x1);
    build.finalize();
    assert_eq!(build.code[0], 0xAA0103E0, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_register_a_64(RegisterA64::w0, RegisterA64::w1);
    build.finalize();
    assert_eq!(build.code[0], 0x2A0103E0, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_register_a_64(RegisterA64::q0, RegisterA64::q1);
    build.finalize();
    assert_eq!(build.code[0], 0x4EA11C20, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movz(RegisterA64::x0, 42, 0);
    build.finalize();
    assert_eq!(build.code[0], 0xD2800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movz(RegisterA64::w0, 42, 0);
    build.finalize();
    assert_eq!(build.code[0], 0x52800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movn(RegisterA64::x0, 42, 0);
    build.finalize();
    assert_eq!(build.code[0], 0x92800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movn(RegisterA64::w0, 42, 0);
    build.finalize();
    assert_eq!(build.code[0], 0x12800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.movk(RegisterA64::x0, 42, 16);
    build.finalize();
    assert_eq!(build.code[0], 0xF2A00540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::x0, 42);
    build.finalize();
    assert_eq!(build.code[0], 0xD2800540, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::x0, 424242);
    build.finalize();
    assert_eq!(build.code[0], 0xD28F2640, "instruction byte mismatch");
    assert_eq!(build.code[1], 0xF2A000C0, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::x0, -42);
    build.finalize();
    assert_eq!(build.code[0], 0x92800520, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::x0, -424242);
    build.finalize();
    assert_eq!(build.code[0], 0x928F2620, "instruction byte mismatch");
    assert_eq!(build.code[1], 0xF2BFFF20, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::x0, -65536);
    build.finalize();
    assert_eq!(build.code[0], 0x929FFFE0, "instruction byte mismatch");

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build.mov_register_a_64_i32(RegisterA64::x0, -65537);
    build.finalize();
    assert_eq!(build.code[0], 0x92800000, "instruction byte mismatch");
    assert_eq!(build.code[1], 0xF2BFFFC0, "instruction byte mismatch");
}
