#[cfg(test)]
#[test]
fn assembly_builder_a_64_constants() {
    use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
    use luaur_code_gen::records::register_a_64::RegisterA64 as R;

    let mut fixture =
        crate::records::assembly_builder_a_64_fixture::AssemblyBuilderA64Fixture::default();

    let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);

    let arr: [u8; 12] = *b"hello world\0";
    let ptr = arr.as_ptr() as *const core::ffi::c_void;
    build.adr_register_a_64_void_usize(R::x0.into(), ptr, 12);

    build.adr_register_a_64_u64(R::x0.into(), 0x1234567887654321u64);

    build.adr_register_a_64_f64(R::x0.into(), 1.0f64);

    build.finalize();

    let expected_code: alloc::vec::Vec<u32> = alloc::vec![0x10ffffa0, 0x10ffff20, 0x10fffec0];
    let expected_data: alloc::vec::Vec<u8> = alloc::vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, 0x21, 0x43, 0x65, 0x87, 0x78, 0x56, 0x34,
        0x12, 0x00, 0x00, 0x00, 0x00, b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l',
        b'd', 0x00,
    ];

    let result = fixture.check(
        |b| {
            let arr: [u8; 12] = *b"hello world\0";
            let ptr = arr.as_ptr() as *const core::ffi::c_void;
            b.adr_register_a_64_void_usize(R::x0.into(), ptr, 12);
            b.adr_register_a_64_u64(R::x0.into(), 0x1234567887654321u64);
            b.adr_register_a_64_f64(R::x0.into(), 1.0f64);
        },
        expected_code,
        expected_data,
        0,
    );

    assert!(result, "Constants check failed");
}
