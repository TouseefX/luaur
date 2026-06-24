#[cfg(test)]
#[test]
fn assembly_builder_x_64_constant_storage() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

    for i in 0..=3000i32 {
        let c = build.i32(i);
        build.vaddss(R::xmm0.into(), R::xmm0.into(), c);
    }

    build.finalize();

    assert_eq!(build.data.len(), 12004);

    for i in 0..=3000i32 {
        let u = i as usize;
        assert_eq!(build.data[u * 4 + 0], ((3000 - i) & 0xff) as u8);
        assert_eq!(build.data[u * 4 + 1], ((3000 - i) >> 8) as u8);
        assert_eq!(build.data[u * 4 + 2], 0x00);
        assert_eq!(build.data[u * 4 + 3], 0x00);
    }
}
