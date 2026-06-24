use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn single_yield_continuation(L: *mut lua_State, _status: i32) -> i32 {
    unsafe {
        lua_pushnumber(L, 4.0);
    }
    1
}
