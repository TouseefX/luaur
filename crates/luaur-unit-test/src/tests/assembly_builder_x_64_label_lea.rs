#[cfg(test)]
#[test]
fn assembly_builder_x_64_label_lea() {
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::label::Label;
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(
        |b| {
            let mut f = Label::default();
            b.lea_register_x_64_label(R::rax, &mut f);
            b.ret();
            b.set_label_label(&mut f);
            b.ret();
        },
        &[0x48, 0x8d, 0x05, 0x01, 0x00, 0x00, 0x00, 0xc3, 0xc3],
    );
}
