#[cfg(test)]
#[test]
fn assembly_builder_a_64_simd_math() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), expected: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], expected, "instruction word mismatch");
    }

    let q0 = RegisterA64::q0;
    let q1 = RegisterA64::q1;
    let q2 = RegisterA64::q2;

    check(|b| b.fadd(q0, q1, q2), 0x4E22D420);
    check(|b| b.fsub(q0, q1, q2), 0x4EA2D420);
    check(|b| b.fmul(q0, q1, q2), 0x6E22DC20);
    check(|b| b.fdiv(q0, q1, q2), 0x6E22FC20);
    check(|b| b.fneg(q0, q1), 0x6EA0F820);
}
