use crate::functions::create_shared_code_gen_context_code_gen_context_alt_c::create_shared_code_gen_context_usize_usize_allocation_callback_void;
use crate::type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext;

pub fn create_shared_code_gen_context() -> UniqueSharedCodeGenContext {
    let block_size = luaur_common::FInt::LuauCodeGenBlockSize.get() as usize;
    let max_total_size = luaur_common::FInt::LuauCodeGenMaxTotalSize.get() as usize;

    create_shared_code_gen_context_usize_usize_allocation_callback_void(
        block_size,
        max_total_size,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    )
}
