use core::ffi::c_char;
use luaur_vm::functions::lua_checkstack::lua_checkstack;
use luaur_vm::functions::lua_pushstring::lua_pushstring;
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn setup_arguments(l: *mut lua_State, argc: i32, argv: *mut *mut c_char) {
    lua_checkstack(l, argc);

    for i in 0..argc {
        let arg = unsafe { *argv.add(i as usize) };
        lua_pushstring(l, arg);
    }
}
