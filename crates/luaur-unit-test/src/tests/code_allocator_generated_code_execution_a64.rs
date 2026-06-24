//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:890:code_allocator_generated_code_execution_a64`
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
//!   - type_ref -> record AssemblyBuilderA64 (CodeGen/include/Luau/AssemblyBuilderA64.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
//!   - calls -> method AssemblyBuilderA64::cbz (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method AssemblyBuilderA64::ldrsw (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method AssemblyBuilderA64::cbnz (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method AssemblyBuilderA64::ldrb (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
//!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
//!   - translates_to -> rust_item code_allocator_generated_code_execution_a64

#[cfg(test)]
#[test]
fn code_allocator_generated_code_execution_a64() {
    #[cfg(not(target_arch = "aarch64"))]
    {
        return;
    }

    #[cfg(target_arch = "aarch64")]
    {
        use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
        use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
        use luaur_code_gen::records::code_allocator::CodeAllocator;
        use luaur_code_gen::records::label::Label;
        use luaur_code_gen::records::register_a_64::RegisterA64;
        use luaur_code_gen::type_aliases::mem::mem;
        use luaur_common::FFlag;

        let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);

        let mut skip = Label::default();
        build.cbz(RegisterA64::x1, &mut skip);
        build.ldrsw(RegisterA64::x1, mem(RegisterA64::x1, 0));
        build.cbnz(RegisterA64::x1, &mut skip);
        build.mov_register_a_64_i32(RegisterA64::x1, 0);
        build.set_label_label(&mut skip);

        let one = 1_u8;
        build.adr_register_a_64_void_usize(
            RegisterA64::x2,
            (&one as *const u8).cast(),
            core::mem::size_of_val(&one),
        );
        build.ldrb(RegisterA64::w2, mem(RegisterA64::x2, 0));
        build.sub_register_a_64_register_a_64_register_a_64_i32(
            RegisterA64::x1,
            RegisterA64::x1,
            RegisterA64::x2,
            0,
        );

        build.add_register_a_64_register_a_64_u16(RegisterA64::x1, RegisterA64::x1, 2);
        build.add_register_a_64_register_a_64_register_a_64_i32(
            RegisterA64::x0,
            RegisterA64::x0,
            RegisterA64::x1,
            1,
        );

        build.ret();
        assert!(build.finalize());

        let block_size = 1024 * 1024;
        let max_total_size = 1024 * 1024;
        let mut allocator = CodeAllocator::default();
        allocator.code_allocator_usize_usize(block_size, max_total_size);

        let code_allocation = allocator.allocate(
            build.data.as_ptr(),
            build.data.len(),
            build.code.as_ptr().cast(),
            build.code.len() * core::mem::size_of::<u32>(),
        );
        assert!(!code_allocation.code_start.is_null());

        type FunctionType = extern "C" fn(i64, *mut core::ffi::c_int) -> i64;
        let f: FunctionType = unsafe { core::mem::transmute(code_allocation.code_start) };

        let mut input = 10;
        let result = f(20, &mut input);
        assert_eq!(result, 42);

        allocator.deallocate(code_allocation);
    }
}
