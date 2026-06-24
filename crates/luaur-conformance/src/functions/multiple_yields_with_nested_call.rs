use core::ffi::c_int;

use luaur_vm::functions::lua_l_callyieldable::lua_l_callyieldable;
use luaur_vm::functions::lua_l_checkboolean::lua_l_checkboolean;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::functions::lua_pushinteger::lua_pushinteger;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::functions::lua_settop::lua_settop;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;
use luaur_vm::type_aliases::lua_continuation::lua_Continuation;

use crate::functions::nested_multiple_yield_helper::nested_multiple_yield_helper;
use crate::functions::nested_multiple_yield_helper_continuation::nested_multiple_yield_helper_continuation;
use crate::functions::nested_multiple_yield_helper_non_yielding::nested_multiple_yield_helper_non_yielding;

#[allow(non_snake_case)]
pub unsafe fn multipleYieldsWithNestedCall(L: *mut lua_State) -> c_int {
    lua_settop(L, 2);
    let nested_should_yield = lua_l_checkboolean(L, 2) != 0;

    lua_pushinteger(L, 0);
    lua_pushnumber(L, 5.0);

    if nested_should_yield {
        let f: lua_CFunction = Some(nested_multiple_yield_helper);
        let cont: lua_Continuation = Some(nested_multiple_yield_helper_continuation);
        lua_pushcclosurek(L, f, core::ptr::null(), 1, cont);
    } else {
        let f: lua_CFunction = Some(core::mem::transmute(
            nested_multiple_yield_helper_non_yielding as fn(*mut lua_State) -> i32,
        ));
        lua_pushcclosurek(L, f, core::ptr::null(), 1, None);
    }

    lua_l_callyieldable(L, 0, 1)
}
