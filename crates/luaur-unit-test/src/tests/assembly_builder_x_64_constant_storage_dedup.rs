#[cfg(test)]
#[test]
fn assembly_builder_x_64_constant_storage_dedup() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    for _ in 0..=3000 {
        let c = build.f32(1.0);
        build.vaddss(R::xmm0.into(), R::xmm0.into(), c);
    }

    build.finalize();

    assert_eq!(build.data.len(), 4);
    assert_eq!(build.data[0], 0x00);
    assert_eq!(build.data[1], 0x00);
    assert_eq!(build.data[2], 0x80);
    assert_eq!(build.data[3], 0x3f);
}
