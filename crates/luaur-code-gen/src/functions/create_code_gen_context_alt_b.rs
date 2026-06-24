use crate::functions::create_code_gen_context_alt_c::create_lua_state_usize_usize_allocation_callback_void;
use crate::type_aliases::allocation_callback::AllocationCallback;
use crate::type_aliases::lua_state::lua_State;

pub fn create_lua_state_allocation_callback_void(
    l: *mut lua_State,
    allocation_callback: *mut AllocationCallback,
    allocation_callback_context: *mut core::ffi::c_void,
) {
    create_lua_state_usize_usize_allocation_callback_void(
        l,
        luaur_common::FInt::LuauCodeGenBlockSize.get() as usize,
        luaur_common::FInt::LuauCodeGenMaxTotalSize.get() as usize,
        allocation_callback,
        allocation_callback_context,
    );
}
