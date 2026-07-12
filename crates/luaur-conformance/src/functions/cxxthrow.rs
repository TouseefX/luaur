use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::macros::lua_use_longjmp::LUA_USE_LONGJMP;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn cxxthrow(L: *mut lua_State) -> i32 {
    unsafe {
        if LUA_USE_LONGJMP != 0 {
            lua_l_error_l(
                L,
                b"oops\0".as_ptr() as *const core::ffi::c_char,
                core::format_args!("oops"),
            );
        } else {
            panic!("oops");
        }
    }
    0
}
