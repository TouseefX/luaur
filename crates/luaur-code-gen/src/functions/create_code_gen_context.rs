use crate::functions::create_code_gen_context_alt_c::create_lua_state_usize_usize_allocation_callback_void;
use crate::type_aliases::lua_state::lua_State;

pub fn create(l: *mut lua_State) {
    let block_size = luaur_common::FInt::LuauCodeGenBlockSize.get() as usize;
    let max_total_size = luaur_common::FInt::LuauCodeGenMaxTotalSize.get() as usize;

    create_lua_state_usize_usize_allocation_callback_void(
        l,
        block_size,
        max_total_size,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
}
