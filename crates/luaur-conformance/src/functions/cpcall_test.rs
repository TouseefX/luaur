use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::functions::lua_tolightuserdata::lua_tolightuserdata;
use luaur_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn cpcallTest(L: *mut luaur_vm::records::lua_state::LuaState) -> core::ffi::c_int {
    let should_fail = *(lua_tolightuserdata(L, 1) as *const bool);

    if should_fail {
        lua_l_error_l(L, c"Failed".as_ptr(), format_args!("Failed"));
    } else {
        lua_pushinteger(L, 123);
        lua_setfield(L, LUA_GLOBALSINDEX, c"cpcallvalue".as_ptr());
    }

    0
}
