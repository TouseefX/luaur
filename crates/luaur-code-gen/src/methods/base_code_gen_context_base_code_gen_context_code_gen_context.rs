use crate::functions::create_block_unwind_info::create_block_unwind_info;
use crate::functions::destroy_block_unwind_info::destroy_block_unwind_info;
use crate::functions::init_functions::init_functions;
use crate::functions::is_supported::is_supported;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::base_code_gen_context::BaseCodeGenContext;
use crate::records::code_allocation_data::CodeAllocationData;
use crate::records::code_allocator::CodeAllocator;
use crate::records::native_context::NativeContext;
use crate::records::unwind_builder::UnwindBuilder;
use crate::type_aliases::allocation_callback::AllocationCallback;
use core::ffi::c_void;

impl crate::records::base_code_gen_context::BaseCodeGenContext {
    pub fn base_code_gen_context_base_code_gen_context(
        block_size: usize,
        max_total_size: usize,
        allocation_callback: *mut AllocationCallback,
        allocation_callback_context: *mut c_void,
    ) -> Self {
        CODEGEN_ASSERT!(is_supported());

        let mut code_allocator = CodeAllocator::default();
        let callback = if allocation_callback.is_null() {
            None
        } else {
            Some(unsafe { *allocation_callback })
        };
        code_allocator.code_allocator_usize_usize_allocation_callback_void(
            block_size,
            max_total_size,
            callback,
            allocation_callback_context,
        );

        let unwind_builder: *mut UnwindBuilder = make_unwind_builder();

        code_allocator.context = unwind_builder.cast();
        code_allocator.create_block_unwind_info = Some(create_block_unwind_info);
        code_allocator.destroy_block_unwind_info = Some(destroy_block_unwind_info);

        let mut context = NativeContext::default();
        init_functions(&mut context);

        Self {
            code_allocator,
            try_bind_existing_module_fn: None,
            bind_module_fn: None,
            unwind_builder,
            gate_data_deprecated: core::ptr::null_mut(),
            gate_data_size_deprecated: 0,
            gate_allocation_data: CodeAllocationData::default(),
            userdata_remapping_context: core::ptr::null_mut(),
            userdata_remapper: None,
            context,
        }
    }
}

#[cfg(target_os = "windows")]
fn make_unwind_builder() -> *mut UnwindBuilder {
    let builder =
        alloc::boxed::Box::new(crate::records::unwind_builder_win::UnwindBuilderWin::default());
    alloc::boxed::Box::into_raw(builder).cast()
}

#[cfg(not(target_os = "windows"))]
fn make_unwind_builder() -> *mut UnwindBuilder {
    let builder = alloc::boxed::Box::new(
        crate::records::unwind_builder_dwarf_2::UnwindBuilderDwarf2::default(),
    );
    alloc::boxed::Box::into_raw(builder).cast()
}
