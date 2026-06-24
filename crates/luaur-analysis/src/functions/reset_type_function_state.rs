use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_call::lua_call;
use luaur_vm::functions::lua_getfield::lua_getfield;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::macros::lua_getglobal::lua_getglobal;
use luaur_vm::macros::lua_pop::lua_pop;

#[allow(non_snake_case)]
pub unsafe fn reset_type_function_state(l: *mut lua_State) {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    lua_getglobal(vm_l, c"math".as_ptr());
    lua_getfield(vm_l, -1, c"randomseed".as_ptr());
    lua_pushnumber(vm_l, 0.0);
    lua_call(vm_l, 1, 0);
    lua_pop(vm_l, 1);
}
