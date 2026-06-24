use crate::functions::initialize_execution_callbacks::initialize_execution_callbacks;
use crate::records::standalone_code_gen_context::StandaloneCodeGenContext;
use crate::type_aliases::allocation_callback::AllocationCallback;
use crate::type_aliases::lua_state::lua_State;

#[allow(non_snake_case)]
extern "C" {
    #[link_name = "_ZN4Luau7CodeGen25StandaloneCodeGenContextC1EmmPFPvS2_S2_mS2_mES2_"]
    fn StandaloneCodeGenContext_ctor(
        this: *mut StandaloneCodeGenContext,
        block_size: usize,
        max_total_size: usize,
        allocation_callback: *mut AllocationCallback,
        allocation_callback_context: *mut core::ffi::c_void,
    );

    #[link_name = "_ZN4Luau7CodeGen18BaseCodeGenContext19initHeaderFunctionsEv"]
    fn BaseCodeGenContext_initHeaderFunctions(this: *mut StandaloneCodeGenContext) -> bool;

    #[link_name = "_ZN4Luau7CodeGen28initializeExecutionCallbacksEP9lua_StatePNS0_18BaseCodeGenContextE"]
    fn initialize_execution_callbacks_impl(
        L: *mut lua_State,
        context: *mut StandaloneCodeGenContext,
    );
}

pub fn create_lua_state_usize_usize_allocation_callback_void(
    l: *mut lua_State,
    block_size: usize,
    max_total_size: usize,
    allocation_callback: *mut AllocationCallback,
    allocation_callback_context: *mut core::ffi::c_void,
) {
    unsafe {
        let layout = core::alloc::Layout::new::<StandaloneCodeGenContext>();
        let ptr = alloc::alloc::alloc(layout) as *mut StandaloneCodeGenContext;
        if ptr.is_null() {
            alloc::alloc::handle_alloc_error(layout);
        }

        StandaloneCodeGenContext_ctor(
            ptr,
            block_size,
            max_total_size,
            allocation_callback,
            allocation_callback_context,
        );

        if !BaseCodeGenContext_initHeaderFunctions(ptr) {
            // Note: In C++, the unique_ptr would delete the object here.
            // We must manually drop/deallocate if init fails.
            // However, the source just returns, so we follow suit with a leak or manual cleanup.
            // For a faithful translation of the unique_ptr behavior:
            core::ptr::drop_in_place(ptr);
            alloc::alloc::dealloc(ptr as *mut u8, layout);
            return;
        }

        initialize_execution_callbacks_impl(l, ptr);
    }
}
