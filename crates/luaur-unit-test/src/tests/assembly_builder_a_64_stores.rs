#[cfg(test)]
#[test]
fn assembly_builder_a_64_stores() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;
    use luaur_code_gen::type_aliases::mem::mem;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    // address forms
    check(
        |b| b.str(RegisterA64::x0, mem(RegisterA64::x1, 0)),
        0xF9000020,
    );
    check(
        |b| b.str(RegisterA64::x0, mem(RegisterA64::x1, 8)),
        0xF9000420,
    );
    check(
        |b| b.str(RegisterA64::x0, mem(RegisterA64::x1, RegisterA64::x7)),
        0xF8276820,
    );
    check(
        |b| b.strh(RegisterA64::w0, mem(RegisterA64::x1, -7)),
        0x781F9020,
    );

    // store sizes
    check(
        |b| b.str(RegisterA64::x0, mem(RegisterA64::x1, 0)),
        0xF9000020,
    );
    check(
        |b| b.str(RegisterA64::w0, mem(RegisterA64::x1, 0)),
        0xB9000020,
    );
    check(
        |b| b.strb(RegisterA64::w0, mem(RegisterA64::x1, 0)),
        0x39000020,
    );
    check(
        |b| b.strh(RegisterA64::w0, mem(RegisterA64::x1, 0)),
        0x79000020,
    );

    // store sizes x offset scaling
    check(
        |b| b.str(RegisterA64::x0, mem(RegisterA64::x1, 8)),
        0xF9000420,
    );
    check(
        |b| b.str(RegisterA64::w0, mem(RegisterA64::x1, 8)),
        0xB9000820,
    );
    check(
        |b| b.strb(RegisterA64::w0, mem(RegisterA64::x1, 8)),
        0x39002020,
    );
    check(
        |b| b.strh(RegisterA64::w0, mem(RegisterA64::x1, 8)),
        0x79001020,
    );

    // paired stores
    check(
        |b| b.stp(RegisterA64::x0, RegisterA64::x1, mem(RegisterA64::x2, 8)),
        0xA9008440,
    );
    check(
        |b| b.stp(RegisterA64::w0, RegisterA64::w1, mem(RegisterA64::x2, -8)),
        0x293F0440,
    );
}
