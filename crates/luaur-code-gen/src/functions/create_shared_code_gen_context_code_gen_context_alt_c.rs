use crate::records::shared_code_gen_context::SharedCodeGenContext;
use crate::type_aliases::allocation_callback::AllocationCallback;
use crate::type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext;

pub fn create_shared_code_gen_context_usize_usize_allocation_callback_void(
    block_size: usize,
    max_total_size: usize,
    allocation_callback: *mut AllocationCallback,
    allocation_callback_context: *mut core::ffi::c_void,
) -> UniqueSharedCodeGenContext {
    unsafe {
        let layout = core::alloc::Layout::new::<SharedCodeGenContext>();
        let ptr = alloc::alloc::alloc(layout) as *mut SharedCodeGenContext;
        if ptr.is_null() {
            alloc::alloc::handle_alloc_error(layout);
        }

        (*ptr).shared_code_gen_context_shared_code_gen_context(
            block_size,
            max_total_size,
            allocation_callback,
            allocation_callback_context,
        );

        if !(*ptr).base.init_header_functions() {
            core::ptr::drop_in_place(ptr);
            alloc::alloc::dealloc(ptr as *mut u8, layout);
            return core::ptr::NonNull::dangling();
        }

        core::ptr::NonNull::new_unchecked(ptr)
    }
}
