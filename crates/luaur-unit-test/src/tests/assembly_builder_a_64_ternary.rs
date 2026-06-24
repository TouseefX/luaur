#[cfg(test)]
#[test]
fn assembly_builder_a_64_ternary() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    check(
        |b| {
            b.msub(
                RegisterA64::x0,
                RegisterA64::x1,
                RegisterA64::x2,
                RegisterA64::x3,
            )
        },
        0x9B028C20,
    );
    check(
        |b| {
            b.msub(
                RegisterA64::w0,
                RegisterA64::w1,
                RegisterA64::w2,
                RegisterA64::w3,
            )
        },
        0x1B028C20,
    );
}
