#[cfg(test)]
#[test]
fn assembly_builder_x_64_control_flow() {
    use luaur_code_gen::enums::condition_x_64::ConditionX64;
    use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
    use luaur_code_gen::records::label::Label;
    use luaur_code_gen::records::register_x_64::RegisterX64 as R;

    fn check(f: impl FnOnce(&mut AssemblyBuilderX64), code: &[u8]) {
        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(&build.code[..], code, "instruction byte mismatch");
    }

    // Jump back (C++ `Label start = build.setLabel();` — no-arg form).
    check(
        |b| {
            let mut start = Label::default();
            b.set_label(&mut start);
            b.add(R::rsi.into(), 1i32.into());
            b.cmp(R::rsi.into(), R::rdi.into());
            b.jcc(ConditionX64::Equal, &mut start);
        },
        &[
            0x48, 0x83, 0xc6, 0x01, 0x48, 0x3b, 0xf7, 0x0f, 0x84, 0xf3, 0xff, 0xff, 0xff,
        ],
    );

    // Jump back, label set before use (C++ in-place `setLabel(start)`).
    check(
        |b| {
            let mut start = Label::default();
            b.add(R::rsi.into(), 1i32.into());
            b.set_label_label(&mut start);
            b.cmp(R::rsi.into(), R::rdi.into());
            b.jcc(ConditionX64::Equal, &mut start);
        },
        &[
            0x48, 0x83, 0xc6, 0x01, 0x48, 0x3b, 0xf7, 0x0f, 0x84, 0xf7, 0xff, 0xff, 0xff,
        ],
    );

    // Jump forward
    check(
        |b| {
            let mut skip = Label::default();
            b.cmp(R::rsi.into(), R::rdi.into());
            b.jcc(ConditionX64::Greater, &mut skip);
            b.or_(R::rdi.into(), 0x3ei32.into());
            b.set_label_label(&mut skip);
        },
        &[
            0x48, 0x3b, 0xf7, 0x0f, 0x8f, 0x04, 0x00, 0x00, 0x00, 0x48, 0x83, 0xcf, 0x3e,
        ],
    );

    // Regular jump
    check(
        |b| {
            let mut skip = Label::default();
            b.jmp_label(&mut skip);
            b.and_(R::rdi.into(), 0x3ei32.into());
            b.set_label_label(&mut skip);
        },
        &[0xe9, 0x04, 0x00, 0x00, 0x00, 0x48, 0x83, 0xe7, 0x3e],
    );
}
