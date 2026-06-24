//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:316:code_allocator_dwarf2_unwind_codes_x64`
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
//!   - translates_to -> rust_item code_allocator_dwarf2_unwind_codes_x64

#[cfg(test)]
#[test]
fn code_allocator_dwarf2_unwind_codes_x64() {
    use luaur_code_gen::records::register_x_64::RegisterX64;
    use luaur_code_gen::records::unwind_builder::UnwindBuilder;
    use luaur_code_gen::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;

    let mut unwind = UnwindBuilderDwarf2::default();

    unwind.start_info(UnwindBuilder::X64);
    unwind.start_function();
    unwind.prologue_x_64(
        23,
        72,
        true,
        &[
            RegisterX64::rdi,
            RegisterX64::rsi,
            RegisterX64::rbx,
            RegisterX64::r12,
            RegisterX64::r13,
            RegisterX64::r14,
            RegisterX64::r15,
        ],
        &[],
    );
    unwind.finish_function(0, 0);
    unwind.finish_info();

    let mut data = vec![0_u8; unwind.get_unwind_info_size(0)];
    unwind.finalize(data.as_mut_ptr().cast(), 0, core::ptr::null_mut(), 0);

    let expected = vec![
        0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x78, 0x10, 0x0c, 0x07,
        0x08, 0x90, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x00, 0x00, 0x1c, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x02, 0x02, 0x0e, 0x10, 0x86, 0x02, 0x02, 0x03, 0x02, 0x02, 0x0e, 0x18,
        0x85, 0x03, 0x02, 0x02, 0x0e, 0x20, 0x84, 0x04, 0x02, 0x02, 0x0e, 0x28, 0x83, 0x05, 0x02,
        0x02, 0x0e, 0x30, 0x8c, 0x06, 0x02, 0x02, 0x0e, 0x38, 0x8d, 0x07, 0x02, 0x02, 0x0e, 0x40,
        0x8e, 0x08, 0x02, 0x02, 0x0e, 0x48, 0x8f, 0x09, 0x02, 0x04, 0x0e, 0x90, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00,
    ];

    assert_eq!(data.len(), expected.len());
    assert_eq!(data, expected);
}
