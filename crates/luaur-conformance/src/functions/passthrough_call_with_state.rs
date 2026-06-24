use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_insert::lua_insert;
use luaur_vm::functions::lua_l_callyieldable::lua_l_callyieldable;
use luaur_vm::functions::lua_l_checkany::lua_l_checkany;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::macros::lua_multret::LUA_MULTRET;
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn passthrough_call_with_state(l: *mut lua_State) -> core::ffi::c_int {
    lua_l_checkany(l, 1);
    let args = lua_gettop(l) - 1;

    lua_pushnumber(l, 42.0);
    lua_insert(l, 1);

    lua_l_callyieldable(l, args, LUA_MULTRET)
}
