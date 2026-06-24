use core::ffi::c_int;
use luaur_vm::functions::lua_l_getmetafield::lua_l_getmetafield;
use luaur_vm::functions::lua_remove::lua_remove;

#[allow(non_snake_case)]
pub unsafe fn try_replace_top_with_index(l: *mut luaur_vm::records::lua_state::lua_State) -> bool {
    if lua_l_getmetafield(l, -1, c"__index".as_ptr()) != 0 {
        // Remove the table leaving __index on the top of stack
        lua_remove(l, -2);
        true
    } else {
        false
    }
}
