use core::ffi::c_int;

use crate::functions::conformance_coverage_callback::conformance_coverage_callback;
use luaur_vm::functions::lua_getcoverage::lua_getcoverage;
use luaur_vm::functions::lua_is_lfunction::lua_is_lfunction;
use luaur_vm::macros::lua_l_argexpected::luaL_argexpected;
use luaur_vm::macros::lua_newtable::lua_newtable;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn conformance_coverage_getcoverage(l: *mut lua_State) -> c_int {
    luaL_argexpected!(l, lua_is_lfunction(l, 1) != 0, 1, "function");

    lua_newtable(l);
    lua_getcoverage(
        l,
        1,
        l as *mut core::ffi::c_void,
        Some(conformance_coverage_callback),
    );

    1
}
