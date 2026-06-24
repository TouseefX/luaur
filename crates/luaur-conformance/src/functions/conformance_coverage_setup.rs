use crate::functions::conformance_coverage_getcoverage::conformance_coverage_getcoverage;
use luaur_vm::macros::lua_pushcfunction::LUA_PUSHCFUNCTION;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_coverage_setup(l: *mut lua_State) {
    LUA_PUSHCFUNCTION(
        l,
        Some(conformance_coverage_getcoverage),
        c"getcoverage".as_ptr(),
    );
    lua_setglobal(l, c"getcoverage".as_ptr());
}
