#[cfg(test)]
#[test]
fn assembly_builder_a_64_stack_ops() {
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
        |b| b.mov_register_a_64_register_a_64(RegisterA64::x0, RegisterA64::sp),
        0x910003E0,
    );
    check(
        |b| b.mov_register_a_64_register_a_64(RegisterA64::sp, RegisterA64::x0),
        0x9100001F,
    );

    check(
        |b| b.add_register_a_64_register_a_64_u16(RegisterA64::sp, RegisterA64::sp, 4),
        0x910013FF,
    );
    check(
        |b| b.sub_register_a_64_register_a_64_u16(RegisterA64::sp, RegisterA64::sp, 4),
        0xD10013FF,
    );

    check(
        |b| b.add_register_a_64_register_a_64_u16(RegisterA64::x0, RegisterA64::sp, 4),
        0x910013E0,
    );
    check(
        |b| b.sub_register_a_64_register_a_64_u16(RegisterA64::sp, RegisterA64::x0, 4),
        0xD100101F,
    );

    check(
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::sp, 8)),
        0xF94007E0,
    );
    check(
        |b| b.str(RegisterA64::x0, mem(RegisterA64::sp, 8)),
        0xF90007E0,
    );
}
