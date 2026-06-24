//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:26:code_allocator_code_allocation`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
//!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
//!   - translates_to -> rust_item code_allocator_code_allocation

#[cfg(test)]
#[test]
fn code_allocator_code_allocation() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::records::code_allocator::CodeAllocator;
    use luaur_common::FFlag;

    const K_CODE_ALIGNMENT: usize = 32;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);
    let _protect_data = ScopedFastFlag::new(&FFlag::LuauCodegenProtectData, false);

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let code = vec![0_u8; 128];

    let result1 = allocator.allocate(core::ptr::null(), 0, code.as_ptr(), code.len());
    assert!(!result1.start.is_null());
    assert_eq!(result1.size, 128);
    assert!(!result1.code_start.is_null());
    assert_eq!(result1.code_start, result1.start);

    let data = vec![0_u8; 8];

    let result2 = allocator.allocate(data.as_ptr(), data.len(), code.as_ptr(), code.len());
    assert!(!result2.start.is_null());
    assert_eq!(result2.size, K_CODE_ALIGNMENT + 128);
    assert!(!result2.code_start.is_null());
    assert_eq!(
        unsafe { result2.start.add(K_CODE_ALIGNMENT) },
        result2.code_start
    );

    allocator.deallocate(result1);
    allocator.deallocate(result2);
}
