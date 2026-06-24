//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:108:code_allocator_code_allocation_failure`
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
//!   - translates_to -> rust_item code_allocator_code_allocation_failure

#[cfg(test)]
#[test]
fn code_allocator_code_allocation_failure() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::records::code_allocator::CodeAllocator;
    use luaur_common::FFlag;

    let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

    let block_size = 3000;
    let max_total_size = 7000;
    let mut allocator = CodeAllocator::default();
    allocator.code_allocator_usize_usize(block_size, max_total_size);

    let mut code = vec![0_u8; 4000];

    let result1 = allocator.allocate(core::ptr::null(), 0, code.as_ptr(), code.len());
    assert!(result1.start.is_null());

    code.resize(2000, 0);
    let result2 = allocator.allocate(core::ptr::null(), 0, code.as_ptr(), code.len());
    assert!(!result2.start.is_null());
    let result3 = allocator.allocate(core::ptr::null(), 0, code.as_ptr(), code.len());
    assert!(!result3.start.is_null());
    let result4 = allocator.allocate(core::ptr::null(), 0, code.as_ptr(), code.len());
    assert!(result4.start.is_null());

    allocator.deallocate(result2);
    allocator.deallocate(result3);
}
