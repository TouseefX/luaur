use luaur_vm::functions::lua_l_callyieldable::lua_l_callyieldable;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn passthrough_call_arg_reuse(l: *mut lua_State) -> i32 {
    lua_l_callyieldable(l, 2, 1)
}
