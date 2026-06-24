#[cfg(test)]
#[test]
fn assembly_builder_x_64_misc_instructions() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(|b| b.int3(), &[0xcc]);
    check(|b| b.ud_2(), &[0x0f, 0x0b]);
    check(|b| b.bsr(R::eax, R::edx.into()), &[0x0f, 0xbd, 0xc2]);
    check(|b| b.bsf(R::eax, R::edx.into()), &[0x0f, 0xbc, 0xc2]);
    check(|b| b.bswap(R::eax), &[0x0f, 0xc8]);
    check(|b| b.bswap(R::r12d), &[0x41, 0x0f, 0xcc]);
    check(|b| b.bswap(R::rax), &[0x48, 0x0f, 0xc8]);
    check(|b| b.bswap(R::r12), &[0x49, 0x0f, 0xcc]);
}
