use core::ffi::c_char;
use luaur_vm::functions::lua_l_getmetafield::lua_l_getmetafield;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn try_replace_top_with_index(l: *mut lua_State) -> bool {
    if lua_l_getmetafield(l, -1, b"__index\0".as_ptr() as *const c_char) != 0 {
        // Remove the table leaving __index on the top of stack
        lua_remove(l, -2);
        true
    } else {
        false
    }
}
