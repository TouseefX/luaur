//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:136:code_allocator_code_allocation_with_unwind_callbacks`
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
//!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
//!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
//!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
//!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
//!   - translates_to -> rust_item code_allocator_code_allocation_with_unwind_callbacks

#[cfg(test)]
#[test]
fn code_allocator_code_allocation_with_unwind_callbacks() {
    use crate::functions::create_block_unwind_info_code_allocator_test::create_block_unwind_info_code_allocator_test;
    use crate::functions::destroy_block_unwind_info_code_allocator_test::destroy_block_unwind_info_code_allocator_test;
    use crate::records::info_code_allocator_test::Info;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::records::code_allocator::CodeAllocator;
    use luaur_common::FFlag;

    const K_CODE_ALIGNMENT: usize = 32;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);
    let _protect_data = ScopedFastFlag::new(&FFlag::LuauCodegenProtectData, false);

    let mut info = Info {
        unwind: vec![0_u8; 8],
        ..Info::default()
    };

    {
        let block_size = 1024 * 1024;
        let max_total_size = 1024 * 1024;
        let mut allocator = CodeAllocator::default();
        allocator.code_allocator_usize_usize(block_size, max_total_size);

        let code = vec![0_u8; 128];
        let data = vec![0_u8; 8];

        allocator.context = (&mut info as *mut Info).cast();
        allocator.create_block_unwind_info = Some(create_block_unwind_info_code_allocator_test);
        allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info_code_allocator_test);

        let result = allocator.allocate(data.as_ptr(), data.len(), code.as_ptr(), code.len());
        assert!(!result.start.is_null());
        assert_eq!(result.size, K_CODE_ALIGNMENT + 128);
        assert!(!result.code_start.is_null());
        assert_eq!(
            unsafe { result.start.add(K_CODE_ALIGNMENT) },
            result.code_start
        );
        assert_eq!(unsafe { info.block.add(K_CODE_ALIGNMENT) }, result.start);

        allocator.deallocate(result);
    }

    assert!(info.destroy_called);
}
