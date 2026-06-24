#[cfg(test)]
#[test]
fn assembly_builder_a_64_address_of_label() {
    use crate::records::assembly_builder_a_64_fixture::AssemblyBuilderA64Fixture;
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::label::Label;
    use luaur_code_gen::records::register_a_64::RegisterA64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: &[u32]) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    check(
        |b| {
            let mut label = Label::default();
            b.adr_register_a_64_label(R::x0, &mut label);
            b.add_register_a_64_register_a_64_register_a_64_i32(R::x0, R::x0, R::x0, 0);
            b.set_label_label(&mut label);
        },
        &[0x10000040, 0x8b000000],
    );
}
