//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:292:code_allocator_windows_unwind_codes_x64`
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
//!   - type_ref -> record UnwindBuilderWin (CodeGen/include/Luau/UnwindBuilderWin.h)
//!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
//!   - translates_to -> rust_item code_allocator_windows_unwind_codes_x64

#[cfg(test)]
#[test]
fn code_allocator_windows_unwind_codes_x64() {
    use luaur_code_gen::records::register_x_64::RegisterX64;
    use luaur_code_gen::records::unwind_builder::UnwindBuilder;
    use luaur_code_gen::records::unwind_builder_win::UnwindBuilderWin;

    let mut unwind = UnwindBuilderWin::default();

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
    unwind.finish_function(0x11223344, 0x55443322);
    unwind.finish_info();

    let mut data = vec![0_u8; unwind.get_unwind_info_size(0)];
    unwind.finalize(data.as_mut_ptr().cast(), 0, core::ptr::null_mut(), 0);

    let expected = vec![
        0x44, 0x33, 0x22, 0x11, 0x22, 0x33, 0x44, 0x55, 0x0c, 0x00, 0x00, 0x00, 0x01, 0x17, 0x0a,
        0x05, 0x17, 0x82, 0x13, 0xf0, 0x11, 0xe0, 0x0f, 0xd0, 0x0d, 0xc0, 0x0b, 0x30, 0x09, 0x60,
        0x07, 0x70, 0x05, 0x03, 0x02, 0x50,
    ];

    assert_eq!(data.len(), expected.len());
    assert_eq!(data, expected);
}
