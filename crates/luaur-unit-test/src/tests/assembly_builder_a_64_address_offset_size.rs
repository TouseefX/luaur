#[cfg(test)]
#[test]
fn assembly_builder_a_64_address_offset_size() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64;
    use luaur_code_gen::type_aliases::mem::mem;

    fn check(f: impl FnOnce(&mut AssemblyBuilderA64), code: u32) {
        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        f(&mut build);
        build.finalize();
        assert_eq!(build.code[0], code, "instruction byte mismatch");
    }

    check(
        |b| b.ldr(RegisterA64::w0, mem(RegisterA64::x1, 16)),
        0xB9401020,
    );
    check(
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::x1, 16)),
        0xF9400820,
    );
    check(
        |b| b.ldr(RegisterA64::d0, mem(RegisterA64::x1, 16)),
        0xFD400820,
    );
    check(
        |b| b.ldr(RegisterA64::q0, mem(RegisterA64::x1, 16)),
        0x3DC00420,
    );

    check(
        |b| b.str(RegisterA64::w0, mem(RegisterA64::x1, 16)),
        0xB9001020,
    );
    check(
        |b| b.str(RegisterA64::x0, mem(RegisterA64::x1, 16)),
        0xF9000820,
    );
    check(
        |b| b.str(RegisterA64::d0, mem(RegisterA64::x1, 16)),
        0xFD000820,
    );
    check(
        |b| b.str(RegisterA64::q0, mem(RegisterA64::x1, 16)),
        0x3D800420,
    );
}
