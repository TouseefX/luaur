//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:432:code_allocator_generated_code_execution_with_throw_x64`
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
//!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
//!   - type_ref -> record UnwindBuilderWin (CodeGen/include/Luau/UnwindBuilderWin.h)
//!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
//!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
//!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
//!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
//!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> function nonthrowing (tests/CodeAllocator.test.cpp)
//!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
//!   - translates_to -> rust_item code_allocator_generated_code_execution_with_throw_x64

#[cfg(test)]
#[test]
fn code_allocator_generated_code_execution_with_throw_x64() {
    #[cfg(not(target_arch = "x86_64"))]
    {
        return;
    }

    #[cfg(target_arch = "x86_64")]
    {
        use crate::functions::assert_code_allocator_testing_panic::assert_code_allocator_testing_panic;
        use crate::functions::nonthrowing::nonthrowing;
        use crate::functions::throwing_code_allocator_test_alt_b::throwing;
        use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
        use luaur_code_gen::functions::create_block_unwind_info::create_block_unwind_info;
        use luaur_code_gen::functions::destroy_block_unwind_info::destroy_block_unwind_info;
        use luaur_code_gen::functions::is_supported::is_supported;
        use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
        use luaur_code_gen::records::code_allocator::CodeAllocator;
        use luaur_code_gen::records::label::Label;
        use luaur_code_gen::records::register_x_64::RegisterX64 as R;
        use luaur_code_gen::records::unwind_builder::UnwindBuilder;
        use luaur_common::FFlag;

        #[cfg(not(windows))]
        use luaur_code_gen::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2;
        #[cfg(windows)]
        use luaur_code_gen::records::unwind_builder_win::UnwindBuilderWin;

        let _free_blocks = ScopedFastFlag::new(&FFlag::LuauCodegenFreeBlocks, true);

        if !is_supported() {
            return;
        }

        #[cfg(windows)]
        let (r_arg1, r_arg2) = (R::rcx, R::rdx);
        #[cfg(not(windows))]
        let (r_arg1, r_arg2) = (R::rdi, R::rsi);

        const R_NON_VOL1: R = R::r12;
        const R_NON_VOL2: R = R::rbx;

        let mut build = AssemblyBuilderX64::assembly_builder_x_64_bool_i32(false, 0);

        #[cfg(windows)]
        let mut unwind = UnwindBuilderWin::default();
        #[cfg(not(windows))]
        let mut unwind = UnwindBuilderDwarf2::default();

        unwind.start_info(UnwindBuilder::X64);

        let mut function_begin = Label::default();
        build.set_label(&mut function_begin);
        unwind.start_function();

        build.push(R::rbp.into());
        build.mov(R::rbp.into(), R::rsp.into());
        build.push(R_NON_VOL1.into());
        build.push(R_NON_VOL2.into());

        let stack_size = 32;
        let locals_size = 16;

        build.sub(R::rsp.into(), (stack_size + locals_size).into());

        let mut prologue_end = Label::default();
        build.set_label(&mut prologue_end);
        let prologue_size = prologue_end.location;

        unwind.prologue_x_64(
            prologue_size,
            stack_size + locals_size,
            true,
            &[R_NON_VOL1, R_NON_VOL2],
            &[],
        );

        build.mov(R_NON_VOL1.into(), r_arg1.into());
        build.mov(R_NON_VOL2.into(), r_arg2.into());

        build.add(R_NON_VOL1.into(), 15.into());
        build.mov(r_arg1.into(), R_NON_VOL1.into());
        build.call_operand_x_64(R_NON_VOL2.into());

        build.add(R::rsp.into(), (stack_size + locals_size).into());
        build.pop(R_NON_VOL2.into());
        build.pop(R_NON_VOL1.into());
        build.pop(R::rbp.into());
        build.ret();

        unwind.finish_function(build.get_label_offset(&function_begin), u32::MAX);

        assert!(build.finalize());
        unwind.finish_info();

        let block_size = 1024 * 1024;
        let max_total_size = 1024 * 1024;
        let mut allocator = CodeAllocator::default();
        allocator.code_allocator_usize_usize(block_size, max_total_size);

        allocator.context = (&mut unwind as *mut _).cast();
        allocator.create_block_unwind_info = Some(create_block_unwind_info);
        allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

        let code_allocation = allocator.allocate(
            build.data.as_ptr(),
            build.data.len(),
            build.code.as_ptr(),
            build.code.len(),
        );
        assert!(!code_allocation.code_start.is_null());

        type FunctionType = extern "C-unwind" fn(i64, extern "C-unwind" fn(i64)) -> i64;
        let f: FunctionType = unsafe { core::mem::transmute(code_allocation.code_start) };

        let _ = f(10, nonthrowing);

        let result = std::panic::catch_unwind(|| {
            let _ = f(10, throwing);
        });

        assert_code_allocator_testing_panic(result.expect_err("expected testing panic"));

        allocator.deallocate(code_allocation);
    }
}
