#[cfg(test)]
#[test]
fn assembly_builder_a_64_mul_div() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    check(|b| b.mul(R::x0, R::x1, R::x2), 0x9B027C20);
    check(|b| b.mul(R::w0, R::w1, R::w2), 0x1B027C20);
    check(|b| b.sdiv(R::x0, R::x1, R::x2), 0x9AC20C20);
    check(|b| b.sdiv(R::w0, R::w1, R::w2), 0x1AC20C20);
    check(|b| b.udiv(R::x0, R::x1, R::x2), 0x9AC20820);
    check(|b| b.udiv(R::w0, R::w1, R::w2), 0x1AC20820);
}
