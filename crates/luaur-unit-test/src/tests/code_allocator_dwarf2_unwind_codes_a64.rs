//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:343:code_allocator_dwarf2_unwind_codes_a64`
//! Source: `tests/CodeAllocator.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/CodeAllocator.test.cpp
//! - source_includes:
//!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderX64.h
//!   - includes -> source_file CodeGen/include/Luau/AssemblyBuilderA64.h
//!   - includes -> source_file CodeGen/include/Luau/CodeAllocator.h
//!   - includes -> source_file CodeGen/include/Luau/CodeBlockUnwind.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file CodeGen/include/Luau/UnwindBuilder.h
//!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderDwarf2.h
//!   - includes -> source_file CodeGen/include/Luau/UnwindBuilderWin.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file VM/src/lstring.h
//! - incoming:
//!   - declares <- source_file tests/CodeAllocator.test.cpp
//! - outgoing:
//!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
//!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
//!   - translates_to -> rust_item code_allocator_dwarf2_unwind_codes_a64

#[cfg(test)]
#[test]
fn code_allocator_dwarf2_unwind_codes_a64() {
    use luaur_code_gen::records::register_a_64::RegisterA64;
    use luaur_code_gen::records::unwind_builder::UnwindBuilder;
    use luaur_code_gen::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;

    let mut unwind = UnwindBuilderDwarf2::default();

    unwind.start_info(UnwindBuilder::A64);
    unwind.start_function();
    unwind.prologue_a_64(
        28,
        64,
        &[
            RegisterA64::x29,
            RegisterA64::x30,
            RegisterA64::x19,
            RegisterA64::x20,
            RegisterA64::x21,
            RegisterA64::x22,
            RegisterA64::x23,
            RegisterA64::x24,
        ],
    );
    unwind.finish_function(0, 32);
    unwind.finish_info();

    let mut data = vec![0_u8; unwind.get_unwind_info_size(0)];
    unwind.finalize(data.as_mut_ptr().cast(), 0, core::ptr::null_mut(), 0);

    let expected = vec![
        0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x78, 0x1e, 0x0c, 0x1f,
        0x00, 0x2c, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04, 0x0e, 0x40, 0x02,
        0x18, 0x9d, 0x08, 0x9e, 0x07, 0x93, 0x06, 0x94, 0x05, 0x95, 0x04, 0x96, 0x03, 0x97, 0x02,
        0x98, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    assert_eq!(data.len(), expected.len());
    assert_eq!(data, expected);
}
