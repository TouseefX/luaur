//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/CodeAllocator.test.cpp:532:code_allocator_generated_code_execution_with_throw_x64_simd`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function isSupported (CodeGen/src/CodeGen.cpp)
//!   - type_ref -> record AssemblyBuilderX64 (CodeGen/include/Luau/AssemblyBuilderX64.h)
//!   - calls -> method CFGFixture::build (tests/ControlFlowGraph.test.cpp)
//!   - type_ref -> record UnwindBuilder (CodeGen/include/Luau/UnwindBuilder.h)
//!   - type_ref -> record UnwindBuilderWin (CodeGen/include/Luau/UnwindBuilderWin.h)
//!   - type_ref -> record UnwindBuilderDwarf2 (CodeGen/include/Luau/UnwindBuilderDwarf2.h)
//!   - type_ref -> record Label (CodeGen/include/Luau/Label.h)
//!   - calls -> method AssemblyBuilderX64::vmovaps (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method AssemblyBuilderX64::vxorpd (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record CodeAllocator (CodeGen/include/Luau/CodeAllocator.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function createBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
//!   - calls -> function destroyBlockUnwindInfo (CodeGen/src/CodeBlockUnwind.cpp)
//!   - type_ref -> record CodeAllocationData (CodeGen/include/Luau/CodeAllocationData.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> function nonthrowing (tests/CodeAllocator.test.cpp)
//!   - calls -> function obscureThrowCase (tests/CodeAllocator.test.cpp)
//!   - calls -> method CodeAllocator::deallocate (CodeGen/src/CodeAllocator.cpp)
//!   - translates_to -> rust_item code_allocator_generated_code_execution_with_throw_x64_simd

#[cfg(test)]
#[test]
fn code_allocator_generated_code_execution_with_throw_x64_simd() {
    #[cfg(not(target_arch = "x86_64"))]
    {
        return;
    }

    #[cfg(target_arch = "x86_64")]
    {
        use crate::functions::nonthrowing::nonthrowing;
        use crate::functions::obscure_throw_case::obscure_throw_case;
        use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
        use luaur_code_gen::enums::abix_64::ABIX64;
        use luaur_code_gen::functions::create_block_unwind_info::create_block_unwind_info;
        use luaur_code_gen::functions::destroy_block_unwind_info::destroy_block_unwind_info;
        use luaur_code_gen::functions::is_supported::is_supported;
        use luaur_code_gen::records::assembly_builder_x_64::AssemblyBuilderX64;
        use luaur_code_gen::records::code_allocator::CodeAllocator;
        use luaur_code_gen::records::label::Label;
        use luaur_code_gen::records::operand_x_64::xmmword;
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

        let stack_size = 32 + 64;
        let locals_size = 16;
        let frame_size = stack_size + locals_size;

        build.push(R_NON_VOL1.into());
        build.push(R_NON_VOL2.into());
        build.push(R::rbp.into());
        build.sub(R::rsp.into(), frame_size.into());

        if build.abi == ABIX64::Windows {
            build.vmovaps(
                xmmword.operator_bracket(R::rsp + (frame_size - 0x40)),
                R::xmm6.into(),
            );
            build.vmovaps(
                xmmword.operator_bracket(R::rsp + (frame_size - 0x30)),
                R::xmm7.into(),
            );
            build.vmovaps(
                xmmword.operator_bracket(R::rsp + (frame_size - 0x20)),
                R::xmm8.into(),
            );
            build.vmovaps(
                xmmword.operator_bracket(R::rsp + (frame_size - 0x10)),
                R::xmm9.into(),
            );
        }

        let mut prologue_end = Label::default();
        build.set_label(&mut prologue_end);
        let prologue_size = prologue_end.location;

        if build.abi == ABIX64::Windows {
            unwind.prologue_x_64(
                prologue_size,
                frame_size as u32,
                false,
                &[R_NON_VOL1, R_NON_VOL2, R::rbp],
                &[R::xmm6, R::xmm7, R::xmm8, R::xmm9],
            );
        } else {
            unwind.prologue_x_64(
                prologue_size,
                frame_size as u32,
                false,
                &[R_NON_VOL1, R_NON_VOL2, R::rbp],
                &[],
            );
        }

        build.vxorpd(R::xmm0.into(), R::xmm0.into(), R::xmm0.into());
        build.vmovsd_operand_x_64_operand_x_64_operand_x_64(
            R::xmm6.into(),
            R::xmm0.into(),
            R::xmm0.into(),
        );
        build.vmovsd_operand_x_64_operand_x_64_operand_x_64(
            R::xmm7.into(),
            R::xmm0.into(),
            R::xmm0.into(),
        );
        build.vmovsd_operand_x_64_operand_x_64_operand_x_64(
            R::xmm8.into(),
            R::xmm0.into(),
            R::xmm0.into(),
        );
        build.vmovsd_operand_x_64_operand_x_64_operand_x_64(
            R::xmm9.into(),
            R::xmm0.into(),
            R::xmm0.into(),
        );

        build.mov(R_NON_VOL1.into(), r_arg1.into());
        build.mov(R_NON_VOL2.into(), r_arg2.into());

        build.add(R_NON_VOL1.into(), 15.into());
        build.mov(r_arg1.into(), R_NON_VOL1.into());
        build.call_operand_x_64(R_NON_VOL2.into());

        if build.abi == ABIX64::Windows {
            build.vmovaps(
                R::xmm6.into(),
                xmmword.operator_bracket(R::rsp + (frame_size - 0x40)),
            );
            build.vmovaps(
                R::xmm7.into(),
                xmmword.operator_bracket(R::rsp + (frame_size - 0x30)),
            );
            build.vmovaps(
                R::xmm8.into(),
                xmmword.operator_bracket(R::rsp + (frame_size - 0x20)),
            );
            build.vmovaps(
                R::xmm9.into(),
                xmmword.operator_bracket(R::rsp + (frame_size - 0x10)),
            );
        }

        build.add(R::rsp.into(), frame_size.into());
        build.pop(R::rbp.into());
        build.pop(R_NON_VOL2.into());
        build.pop(R_NON_VOL1.into());
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
        obscure_throw_case(f);

        allocator.deallocate(code_allocation);
    }
}
