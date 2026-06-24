//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:57:code_allocator_code_allocation_callbacks`
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
//!   - type_ref -> record AllocationData (tests/CodeAllocator.test.cpp)
//!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
//!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
//!   - translates_to -> rust_item code_allocator_code_allocation_callbacks

#[cfg(test)]
#[test]
fn code_allocator_code_allocation_callbacks() {
    use crate::functions::allocation_callback_code_allocator_test::allocation_callback_code_allocator_test;
    use crate::records::allocation_data::AllocationData;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::records::code_allocator::CodeAllocator;
    use luaur_common::FFlag;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

    let block_size = 1024 * 1024;
    let max_total_size = 1024 * 1024;
    let mut allocation_data = AllocationData::default();

    {
        let mut allocator = CodeAllocator::default();
        allocator.code_allocator_usize_usize_allocation_callback_void(
            block_size,
            max_total_size,
            Some(allocation_callback_code_allocator_test),
            (&mut allocation_data as *mut AllocationData).cast(),
        );

        let code = vec![0_u8; 128];

        let result = allocator.allocate(core::ptr::null(), 0, code.as_ptr(), code.len());
        assert!(!result.start.is_null());
        assert_eq!(allocation_data.bytes_allocated, block_size);
        assert_eq!(allocation_data.bytes_freed, 0);

        allocator.deallocate(result);
    }

    assert_eq!(allocation_data.bytes_allocated, block_size);
    assert_eq!(allocation_data.bytes_freed, block_size);
}
