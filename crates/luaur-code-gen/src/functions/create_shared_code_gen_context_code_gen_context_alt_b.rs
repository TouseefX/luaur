use crate::functions::create_shared_code_gen_context_code_gen_context_alt_c::create_shared_code_gen_context_usize_usize_allocation_callback_void;

pub fn create_shared_code_gen_context(
    allocation_callback: *mut crate::type_aliases::allocation_callback::AllocationCallback,
    allocation_callback_context: *mut core::ffi::c_void,
) -> crate::type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext {
    create_shared_code_gen_context_usize_usize_allocation_callback_void(
        luaur_common::FInt::LuauCodeGenBlockSize.get() as usize,
        luaur_common::FInt::LuauCodeGenMaxTotalSize.get() as usize,
        allocation_callback,
        allocation_callback_context,
    )
}
