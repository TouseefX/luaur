//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:387:code_allocator_generated_code_execution_x64`
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
//!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
//!   - type_ref -> record AssemblyBuilderX64 (CodeGen/include/Luau/AssemblyBuilderX64.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
//!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
//!   - translates_to -> rust_item code_allocator_generated_code_execution_x64

#[cfg(test)]
#[test]
fn code_allocator_generated_code_execution_x64() {
    #[cfg(not(target_arch = "x86_64"))]
    {
        return;
    }

    #[cfg(target_arch = "x86_64")]
    {
        use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
        use luaur_code_gen::functions::is_supported::is_supported;
        use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
        use luaur_code_gen::records::code_allocator::CodeAllocator;
        use luaur_code_gen::records::register_x_64::RegisterX64 as R;
        use luaur_common::FFlag;

        let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

        if !is_supported() {
            return;
        }

        #[cfg(windows)]
        let (r_arg1, r_arg2) = (R::rcx, R::rdx);
        #[cfg(not(windows))]
        let (r_arg1, r_arg2) = (R::rdi, R::rsi);

        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

        build.mov(R::rax.into(), r_arg1.into());
        build.add(R::rax.into(), r_arg2.into());
        build.imul_operand_x_64_operand_x_64_i32(R::rax.into(), R::rax.into(), 7);
        build.ret();
        assert!(build.finalize());

        let block_size = 1024 * 1024;
        let max_total_size = 1024 * 1024;
        let mut allocator = CodeAllocator::default();
        allocator.code_allocator_usize_usize(block_size, max_total_size);

        let code_allocation = allocator.allocate(
            build.data.as_ptr(),
            build.data.len(),
            build.code.as_ptr(),
            build.code.len(),
        );
        assert!(!code_allocation.code_start.is_null());

        type FunctionType = extern "C" fn(i64, i64) -> i64;
        let f: FunctionType = unsafe { core::mem::transmute(code_allocation.code_start) };
        let result = f(10, 20);
        assert_eq!(result, 210);

        allocator.deallocate(code_allocation);
    }
}
