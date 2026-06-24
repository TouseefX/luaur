use core::ffi::{c_int, c_void};

use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
use luaur_vm::functions::lua_g_isnative::luaG_isnative;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

use crate::functions::run_conformance::CODEGEN;

unsafe extern "C" fn is_native(L: *mut lua_State) -> c_int {
    lua_pushboolean(L, luaG_isnative(L, 1));
    1
}

unsafe extern "C" fn is_native_if_supported(L: *mut lua_State) -> c_int {
    if !CODEGEN || luau_codegen_supported() == 0 {
        lua_pushboolean(L, 1);
    } else {
        lua_pushboolean(L, luaG_isnative(L, 1));
    }

    1
}

pub unsafe extern "C" fn setup_native_helpers(L: *mut lua_State) {
    let is_native_fn: lua_CFunction = Some(core::mem::transmute(
        is_native as unsafe extern "C" fn(*mut lua_State) -> c_int,
    ));
    lua_pushcclosurek(L, is_native_fn, c"is_native".as_ptr(), 0, None);
    lua_setglobal(L, c"is_native".as_ptr());

    let is_native_if_supported_fn: lua_CFunction = Some(core::mem::transmute(
        is_native_if_supported as unsafe extern "C" fn(*mut lua_State) -> c_int,
    ));
    lua_pushcclosurek(
        L,
        is_native_if_supported_fn,
        c"is_native_if_supported".as_ptr(),
        0,
        None,
    );
    lua_setglobal(L, c"is_native_if_supported".as_ptr());
}
