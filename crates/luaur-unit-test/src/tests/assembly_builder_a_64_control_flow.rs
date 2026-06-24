#[cfg(test)]
#[test]
fn assembly_builder_a_64_control_flow() {
    use crate::records::assembly_builder_a_64_fixture::AssemblyBuilderA64Fixture;
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::label::Label;
    use luaur_code_gen::records::register_a_64::RegisterA64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: &[u32]) -> bool {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        &build.code[..] == code
    }

    // Jump back
    let mut start = Label::default();
    let result0 = check(
        |b| {
            start = b.set_label();
            b.mov_register_a_64_register_a_64(R::x0, R::x1);
            b.b_condition_a_64_label(
                luaur_code_gen::enums::condition_a_64::ConditionA64::Equal,
                &mut start,
            );
        },
        &[0xAA0103E0, 0x54FFFFE0],
    );
    assert!(result0, "Jump back check failed");

    // Jump forward
    let mut skip = Label::default();
    let result1 = check(
        |b| {
            b.b_condition_a_64_label(
                luaur_code_gen::enums::condition_a_64::ConditionA64::Equal,
                &mut skip,
            );
            b.mov_register_a_64_register_a_64(R::x0, R::x1);
            b.set_label_label(&mut skip);
        },
        &[0x54000040, 0xAA0103E0],
    );
    assert!(result1, "Jump forward check failed");

    // Jumps
    let mut skip2 = Label::default();
    let result2 = check(
        |b| {
            b.b_condition_a_64_label(
                luaur_code_gen::enums::condition_a_64::ConditionA64::Equal,
                &mut skip2,
            );
            b.cbz(R::x0, &mut skip2);
            b.cbnz(R::x0, &mut skip2);
            b.tbz(R::x0, 5, &mut skip2);
            b.tbnz(R::x0, 5, &mut skip2);
            b.set_label_label(&mut skip2);
            b.b_label(&mut skip2);
            b.bl(&mut skip2);
        },
        &[
            0x540000A0, 0xB4000080, 0xB5000060, 0x36280040, 0x37280020, 0x14000000, 0x97ffffff,
        ],
    );
    assert!(result2, "Jumps check failed");

    // Basic control flow
    let mut build3 = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build3.br(R::x0);
    build3.finalize();
    assert_eq!(build3.code[0], 0xD61F0000u32, "br(x0) instruction mismatch");

    let mut build4 = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build4.blr(R::x0);
    build4.finalize();
    assert_eq!(
        build4.code[0], 0xD63F0000u32,
        "blr(x0) instruction mismatch"
    );

    let mut build5 = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
    build5.ret();
    build5.finalize();
    assert_eq!(build5.code[0], 0xD65F03C0u32, "ret() instruction mismatch");
}
