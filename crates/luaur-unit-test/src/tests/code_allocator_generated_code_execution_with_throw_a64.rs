//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:941:code_allocator_generated_code_execution_with_throw_a64`
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
//!   - calls -> function isUnwindSupported (CodeGen/src/CodeBlockUnwind.cpp)
//!   - type_ref -> record AssemblyBuilderA64 (CodeGen/include/Luau/AssemblyBuilderA64.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
//!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
//!   - calls -> method AssemblyBuilderA64::stp (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> type_alias mem (CodeGen/include/Luau/AddressA64.h)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
//!   - calls -> method AssemblyBuilderA64::blr (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method AssemblyBuilderA64::ldr (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method AssemblyBuilderA64::ldp (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
//!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
//!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
//!   - translates_to -> rust_item code_allocator_generated_code_execution_with_throw_a64

#[cfg(test)]
#[test]
fn code_allocator_generated_code_execution_with_throw_a64() {
    #[cfg(not(target_arch = "aarch64"))]
    {
        return;
    }

    #[cfg(target_arch = "aarch64")]
    {
        use crate::functions::assert_code_allocator_testing_panic::assert_code_allocator_testing_panic;
        use crate::functions::throwing_code_allocator_test_alt_b::throwing;
        use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
        use luaur_code_gen::functions::create_block_unwind_info::create_block_unwind_info;
        use luaur_code_gen::functions::destroy_block_unwind_info::destroy_block_unwind_info;
        use luaur_code_gen::functions::is_unwind_supported::is_unwind_supported;
        use luaur_code_gen::records::assembly_builder_a_64::AssemblyBuilderA64;
        use luaur_code_gen::records::code_allocator::CodeAllocator;
        use luaur_code_gen::records::register_a_64::RegisterA64;
        use luaur_code_gen::records::unwind_builder::UnwindBuilder;
        use luaur_code_gen::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
        use luaur_code_gen::type_aliases::mem::mem;
        use luaur_common::FFlag;

        let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

        if !is_unwind_supported() {
            return;
        }

        let mut build = AssemblyBuilderA64::assembly_builder_a_64_bool_i32(false, 0);
        let mut unwind = UnwindBuilderDwarf2::default();

        unwind.start_info(UnwindBuilder::A64);

        build.sub_register_a_64_register_a_64_u16(RegisterA64::sp, RegisterA64::sp, 32);
        build.stp(RegisterA64::x29, RegisterA64::x30, mem(RegisterA64::sp, 0));
        build.str(RegisterA64::x28, mem(RegisterA64::sp, 16));
        build.mov_register_a_64_register_a_64(RegisterA64::x29, RegisterA64::sp);

        let prologue_end = build.set_label();

        build.add_register_a_64_register_a_64_u16(RegisterA64::x0, RegisterA64::x0, 15);
        build.blr(RegisterA64::x1);

        build.ldr(RegisterA64::x28, mem(RegisterA64::sp, 16));
        build.ldp(RegisterA64::x29, RegisterA64::x30, mem(RegisterA64::sp, 0));
        build.add_register_a_64_register_a_64_u16(RegisterA64::sp, RegisterA64::sp, 32);
        build.ret();

        let function_end = build.set_label();

        unwind.start_function();
        unwind.prologue_a_64(
            build.get_label_offset(&prologue_end),
            32,
            &[RegisterA64::x29, RegisterA64::x30, RegisterA64::x28],
        );
        unwind.finish_function(0, build.get_label_offset(&function_end));

        assert!(build.finalize());
        unwind.finish_info();

        let block_size = 1024 * 1024;
        let max_total_size = 1024 * 1024;
        let mut allocator = CodeAllocator::default();
        allocator.code_allocator_usize_usize(block_size, max_total_size);

        allocator.context = (&mut unwind as *mut UnwindBuilderDwarf2).cast();
        allocator.create_block_unwind_info = Some(create_block_unwind_info);
        allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

        let code_allocation = allocator.allocate(
            build.data.as_ptr(),
            build.data.len(),
            build.code.as_ptr().cast(),
            build.code.len() * core::mem::size_of::<u32>(),
        );
        assert!(!code_allocation.code_start.is_null());

        type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
        let f: FunctionType = unsafe { core::mem::transmute(code_allocation.code_start) };

        let result = std::panic::catch_unwind(|| {
            let _ = f(10, throwing);
        });

        assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

        allocator.deallocate(code_allocation);
    }
}
