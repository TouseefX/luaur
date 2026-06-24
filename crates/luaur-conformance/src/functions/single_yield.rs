use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::functions::lua_yield::lua_yield;
use luaur_vm::records::lua_state::lua_State;

pub fn single_yield(L: *mut lua_State) -> i32 {
    unsafe {
        lua_pushnumber(L, 2.0);
        lua_yield(L, 1)
    }
}
