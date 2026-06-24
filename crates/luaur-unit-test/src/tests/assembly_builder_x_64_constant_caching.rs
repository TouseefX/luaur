#[cfg(test)]
#[test]
fn assembly_builder_x_64_constant_caching() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    let two = build.f64(2.0);

    // Force data relocation
    for i in 0..4096 {
        build.f64(i as f64);
    }

    assert_eq!(build.f64(2.0).imm, two.imm);

    build.finalize();
}
