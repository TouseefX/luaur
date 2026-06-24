#[cfg(test)]
#[test]
fn assembly_builder_a_64_loads() {
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
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::x1, 0)),
        0xF9400020,
    );
    check(
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::x1, 8)),
        0xF9400420,
    );
    check(
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::x1, RegisterA64::x7)),
        0xF8676820,
    );
    check(
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::x1, -7)),
        0xF85F9020,
    );

    // load sizes
    check(
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::x1, 0)),
        0xF9400020,
    );
    check(
        |b| b.ldr(RegisterA64::w0, mem(RegisterA64::x1, 0)),
        0xB9400020,
    );
    check(
        |b| b.ldrb(RegisterA64::w0, mem(RegisterA64::x1, 0)),
        0x39400020,
    );
    check(
        |b| b.ldrh(RegisterA64::w0, mem(RegisterA64::x1, 0)),
        0x79400020,
    );
    check(
        |b| b.ldrsb(RegisterA64::x0, mem(RegisterA64::x1, 0)),
        0x39800020,
    );
    check(
        |b| b.ldrsb(RegisterA64::w0, mem(RegisterA64::x1, 0)),
        0x39C00020,
    );
    check(
        |b| b.ldrsh(RegisterA64::x0, mem(RegisterA64::x1, 0)),
        0x79800020,
    );
    check(
        |b| b.ldrsh(RegisterA64::w0, mem(RegisterA64::x1, 0)),
        0x79C00020,
    );
    check(
        |b| b.ldrsw(RegisterA64::x0, mem(RegisterA64::x1, 0)),
        0xB9800020,
    );

    // load sizes x offset scaling
    check(
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::x1, 8)),
        0xF9400420,
    );
    check(
        |b| b.ldr(RegisterA64::w0, mem(RegisterA64::x1, 8)),
        0xB9400820,
    );
    check(
        |b| b.ldrb(RegisterA64::w0, mem(RegisterA64::x1, 8)),
        0x39402020,
    );
    check(
        |b| b.ldrh(RegisterA64::w0, mem(RegisterA64::x1, 8)),
        0x79401020,
    );
    check(
        |b| b.ldrsb(RegisterA64::w0, mem(RegisterA64::x1, 8)),
        0x39C02020,
    );
    check(
        |b| b.ldrsh(RegisterA64::w0, mem(RegisterA64::x1, 8)),
        0x79C01020,
    );

    // paired loads
    check(
        |b| b.ldp(RegisterA64::x0, RegisterA64::x1, mem(RegisterA64::x2, 8)),
        0xA9408440,
    );
    check(
        |b| b.ldp(RegisterA64::w0, RegisterA64::w1, mem(RegisterA64::x2, -8)),
        0x297F0440,
    );
}
