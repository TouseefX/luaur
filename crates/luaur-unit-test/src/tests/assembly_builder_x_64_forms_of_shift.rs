#[cfg(test)]
#[test]
fn assembly_builder_x_64_forms_of_shift() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(|b| b.shl(R::al.into(), 1i32.into()), &[0xd0, 0xe0]);
    check(|b| b.shl(R::al.into(), R::cl.into()), &[0xd2, 0xe0]);
    check(|b| b.shl(R::sil.into(), R::cl.into()), &[0x40, 0xd2, 0xe6]);
    check(|b| b.shl(R::r10b.into(), R::cl.into()), &[0x41, 0xd2, 0xe2]);
    check(|b| b.shr(R::al.into(), 4i32.into()), &[0xc0, 0xe8, 0x04]);
    check(|b| b.shr(R::eax.into(), 1i32.into()), &[0xd1, 0xe8]);
    check(|b| b.sal(R::eax.into(), R::cl.into()), &[0xd3, 0xe0]);
    check(|b| b.sal(R::eax.into(), 4i32.into()), &[0xc1, 0xe0, 0x04]);
    check(
        |b| b.sar(R::rax.into(), 4i32.into()),
        &[0x48, 0xc1, 0xf8, 0x04],
    );
    check(|b| b.sar(R::r11.into(), 1i32.into()), &[0x49, 0xd1, 0xfb]);
    check(|b| b.rol(R::eax.into(), 1i32.into()), &[0xd1, 0xc0]);
    check(|b| b.rol(R::eax.into(), R::cl.into()), &[0xd3, 0xc0]);
    check(|b| b.ror(R::eax.into(), 1i32.into()), &[0xd1, 0xc8]);
    check(|b| b.ror(R::eax.into(), R::cl.into()), &[0xd3, 0xc8]);
}
