use luaur_vm::macros::lua_pushcfunction::LUA_PUSHCFUNCTION;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

use crate::functions::conformance_tables_make_lud::conformance_tables_make_lud;

pub unsafe extern "C" fn conformance_tables_setup(L: *mut lua_State) {
    let make_lud: lua_CFunction = Some(core::mem::transmute(
        conformance_tables_make_lud as unsafe extern "C" fn(*mut lua_State) -> core::ffi::c_int,
    ));
    LUA_PUSHCFUNCTION(L, make_lud, c"makelud".as_ptr());
    lua_setglobal(L, c"makelud".as_ptr());
}
