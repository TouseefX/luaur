use luaur_vm::functions::lua_l_callyieldable::lua_l_callyieldable;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn passthrough_call(L: *mut lua_State) -> i32 {
    lua_l_checkstack(L, 3, "cpass");
    lua_pushvalue(L, 1);
    lua_pushvalue(L, 2);
    lua_pushvalue(L, 3);
    lua_l_callyieldable(L, 2, 1)
}
