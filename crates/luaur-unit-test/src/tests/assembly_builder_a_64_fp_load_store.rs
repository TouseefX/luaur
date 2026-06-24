#[cfg(test)]
#[test]
fn assembly_builder_a_64_fp_load_store() {
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
        |b| b.ldr(RegisterA64::d0, mem(RegisterA64::x1, 0)),
        0xFD400020,
    );
    check(
        |b| b.ldr(RegisterA64::d0, mem(RegisterA64::x1, 8)),
        0xFD400420,
    );
    check(
        |b| b.ldr(RegisterA64::d0, mem(RegisterA64::x1, RegisterA64::x7)),
        0xFC676820,
    );
    check(
        |b| b.ldr(RegisterA64::d0, mem(RegisterA64::x1, -7)),
        0xFC5F9020,
    );
    check(
        |b| b.str(RegisterA64::d0, mem(RegisterA64::x1, 0)),
        0xFD000020,
    );
    check(
        |b| b.str(RegisterA64::d0, mem(RegisterA64::x1, 8)),
        0xFD000420,
    );
    check(
        |b| b.str(RegisterA64::d0, mem(RegisterA64::x1, RegisterA64::x7)),
        0xFC276820,
    );
    check(
        |b| b.str(RegisterA64::d0, mem(RegisterA64::x1, -7)),
        0xFC1F9020,
    );

    // load/store sizes
    check(
        |b| b.ldr(RegisterA64::s0, mem(RegisterA64::x1, 0)),
        0xBD400020,
    );
    check(
        |b| b.ldr(RegisterA64::d0, mem(RegisterA64::x1, 0)),
        0xFD400020,
    );
    check(
        |b| b.ldr(RegisterA64::q0, mem(RegisterA64::x1, 0)),
        0x3DC00020,
    );
    check(
        |b| b.str(RegisterA64::s0, mem(RegisterA64::x1, 0)),
        0xBD000020,
    );
    check(
        |b| b.str(RegisterA64::d0, mem(RegisterA64::x1, 0)),
        0xFD000020,
    );
    check(
        |b| b.str(RegisterA64::q0, mem(RegisterA64::x1, 0)),
        0x3D800020,
    );

    // load/store sizes x offset scaling
    check(
        |b| b.ldr(RegisterA64::q0, mem(RegisterA64::x1, 16)),
        0x3DC00420,
    );
    check(
        |b| b.ldr(RegisterA64::d0, mem(RegisterA64::x1, 16)),
        0xFD400820,
    );
    check(
        |b| b.ldr(RegisterA64::s0, mem(RegisterA64::x1, 16)),
        0xBD401020,
    );
    check(
        |b| b.str(RegisterA64::q0, mem(RegisterA64::x1, 16)),
        0x3D800420,
    );
    check(
        |b| b.str(RegisterA64::d0, mem(RegisterA64::x1, 16)),
        0xFD000820,
    );
    check(
        |b| b.str(RegisterA64::s0, mem(RegisterA64::x1, 16)),
        0xBD001020,
    );
}
