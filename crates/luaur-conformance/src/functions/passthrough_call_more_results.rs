use luaur_vm::functions::lua_l_callyieldable::lua_l_callyieldable;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn passthrough_call_more_results(l: *mut lua_State) -> i32 {
    lua_l_checkstack(l, 3, "cpass");
    lua_pushvalue(l, 1);
    lua_pushvalue(l, 2);
    lua_pushvalue(l, 3);
    lua_l_callyieldable(l, 2, 10)
}
