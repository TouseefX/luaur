use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::macros::luai_maxcstack::LUAI_MAXCSTACK;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn slowly_overflow_stack(L: *mut lua_State) -> i32 {
    for _ in 0..(LUAI_MAXCSTACK * 2) {
        unsafe {
            lua_l_checkstack(L, 1, "test");
            lua_pushnumber(L, 1.0);
        }
    }
    0
}
