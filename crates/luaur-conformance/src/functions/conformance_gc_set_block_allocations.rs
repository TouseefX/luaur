use core::ffi::c_int;

use crate::functions::blockable_realloc_allowed::blockableReallocAllowed;
use luaur_vm::functions::lua_l_checkboolean::lua_l_checkboolean;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn conformance_gc_set_block_allocations(l: *mut lua_State) -> c_int {
    unsafe {
        blockableReallocAllowed = lua_l_checkboolean(l, 1) == 0;
    }

    0
}
