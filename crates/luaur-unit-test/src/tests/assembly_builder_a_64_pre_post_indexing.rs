#[cfg(test)]
#[test]
fn assembly_builder_a_64_pre_post_indexing() {
    use luaur_code_gen::enums::address_kind_a_64::AddressKindA64;
    use luaur_code_gen::records::address_a_64::AddressA64;
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
        |b| b.ldr(RegisterA64::x0, mem(RegisterA64::x1, 1)),
        0xF8401020,
    );
    check(
        |b| {
            b.ldr(
                RegisterA64::x0,
                AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
                    RegisterA64::x1,
                    1,
                    AddressKindA64::pre,
                ),
            )
        },
        0xF8401C20,
    );
    check(
        |b| {
            b.ldr(
                RegisterA64::x0,
                AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
                    RegisterA64::x1,
                    1,
                    AddressKindA64::post,
                ),
            )
        },
        0xF8401420,
    );

    check(
        |b| b.ldr(RegisterA64::q0, mem(RegisterA64::x1, 1)),
        0x3CC01020,
    );
    check(
        |b| {
            b.ldr(
                RegisterA64::q0,
                AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
                    RegisterA64::x1,
                    1,
                    AddressKindA64::pre,
                ),
            )
        },
        0x3CC01C20,
    );
    check(
        |b| {
            b.ldr(
                RegisterA64::q0,
                AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
                    RegisterA64::x1,
                    1,
                    AddressKindA64::post,
                ),
            )
        },
        0x3CC01420,
    );

    check(
        |b| b.str(RegisterA64::x0, mem(RegisterA64::x1, 1)),
        0xF8001020,
    );
    check(
        |b| {
            b.str(
                RegisterA64::x0,
                AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
                    RegisterA64::x1,
                    1,
                    AddressKindA64::pre,
                ),
            )
        },
        0xF8001C20,
    );
    check(
        |b| {
            b.str(
                RegisterA64::x0,
                AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
                    RegisterA64::x1,
                    1,
                    AddressKindA64::post,
                ),
            )
        },
        0xF8001420,
    );

    check(
        |b| b.str(RegisterA64::q0, mem(RegisterA64::x1, 1)),
        0x3C801020,
    );
    check(
        |b| {
            b.str(
                RegisterA64::q0,
                AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
                    RegisterA64::x1,
                    1,
                    AddressKindA64::pre,
                ),
            )
        },
        0x3C801C20,
    );
    check(
        |b| {
            b.str(
                RegisterA64::q0,
                AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
                    RegisterA64::x1,
                    1,
                    AddressKindA64::post,
                ),
            )
        },
        0x3C801420,
    );
}
