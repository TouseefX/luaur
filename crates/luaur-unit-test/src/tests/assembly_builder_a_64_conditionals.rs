#[cfg(test)]
#[test]
fn assembly_builder_a_64_conditionals() {
    use luaur_code_gen::enums::condition_a_64::ConditionA64;
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    check(
        |b| b.csel(R::x0, R::x1, R::x2, ConditionA64::Equal),
        0x9A820020,
    );
    check(
        |b| b.csel(R::w0, R::w1, R::w2, ConditionA64::Equal),
        0x1A820020,
    );
    check(
        |b| b.fcsel(R::d0, R::d1, R::d2, ConditionA64::Equal),
        0x1E620C20,
    );

    check(|b| b.cset(R::x1, ConditionA64::Less), 0x9A9FA7E1);
}
