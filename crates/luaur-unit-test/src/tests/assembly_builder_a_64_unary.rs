#[cfg(test)]
#[test]
fn assembly_builder_a_64_unary() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction word mismatch");
    }

    check(|b| b.neg(RegisterA64::x0, RegisterA64::x1), 0xCB0103E0);
    check(|b| b.neg(RegisterA64::w0, RegisterA64::w1), 0x4B0103E0);
    check(|b| b.mvn_(RegisterA64::x0, RegisterA64::x1), 0xAA2103E0);

    check(|b| b.clz(RegisterA64::x0, RegisterA64::x1), 0xDAC01020);
    check(|b| b.clz(RegisterA64::w0, RegisterA64::w1), 0x5AC01020);
    check(|b| b.rbit(RegisterA64::x0, RegisterA64::x1), 0xDAC00020);
    check(|b| b.rbit(RegisterA64::w0, RegisterA64::w1), 0x5AC00020);
    check(|b| b.rev(RegisterA64::w0, RegisterA64::w1), 0x5AC00820);
    check(|b| b.rev(RegisterA64::x0, RegisterA64::x1), 0xDAC00C20);
}
