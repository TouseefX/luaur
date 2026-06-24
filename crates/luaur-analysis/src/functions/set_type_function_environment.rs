//! Faithful port of `void setTypeFunctionEnvironment(lua_State* L)`
//! (Analysis/src/TypeFunctionRuntime.cpp:2098-2138).
//!
//! Adds libraries / globals for the type-function environment: opens a curated
//! subset of the standard libraries, replaces a handful of unavailable base
//! globals with a stub that errors, and installs the custom `print`.
use crate::functions::print::print;
use crate::functions::unsupported_function::unsupported_function;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::luaopen_base::luaopen_base;
use luaur_vm::functions::luaopen_bit_32::luaopen_bit32;
use luaur_vm::functions::luaopen_buffer::luaopen_buffer;
use luaur_vm::functions::luaopen_math::luaopen_math;
use luaur_vm::functions::luaopen_string::luaopen_string;
use luaur_vm::functions::luaopen_table::luaopen_table;
use luaur_vm::functions::luaopen_utf_8::luaopen_utf_8;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_pushcfunction::LUA_PUSHCFUNCTION;
use luaur_vm::macros::lua_setglobal::lua_setglobal;

/// `lua_CFunction` thunk for `unsupportedFunction`. Bridges the analysis-level
/// function (over the `c_void` `lua_State` alias) to the VM's `lua_CFunction`
/// shape (`unsafe fn(*mut vm::lua_State) -> c_int`).
unsafe fn unsupported_function_thunk(
    l: *mut luaur_vm::records::lua_state::lua_State,
) -> core::ffi::c_int {
    unsupported_function(l as *mut lua_State)
}

/// `lua_CFunction` thunk for `print`.
unsafe fn print_thunk(l: *mut luaur_vm::records::lua_state::lua_State) -> core::ffi::c_int {
    print(l as *mut lua_State)
}

pub unsafe fn set_type_function_environment(l: *mut lua_State) {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;

    // Register math library
    luaopen_math(vm_l);
    lua_pop(vm_l, 1);

    // Register table library
    luaopen_table(vm_l);
    lua_pop(vm_l, 1);

    // Register string library
    luaopen_string(vm_l);
    lua_pop(vm_l, 1);

    // Register bit32 library
    luaopen_bit32(vm_l);
    lua_pop(vm_l, 1);

    // Register utf8 library
    luaopen_utf_8(vm_l);
    lua_pop(vm_l, 1);

    // Register buffer library
    luaopen_buffer(vm_l);
    lua_pop(vm_l, 1);

    // Register base library
    luaopen_base(vm_l);
    lua_pop(vm_l, 1);

    // Remove certain global functions from the base library
    // static const char* unavailableGlobals[] = {"gcinfo", "getfenv", "newproxy", "setfenv", "pcall", "xpcall"};
    let unavailable_globals: [*const core::ffi::c_char; 6] = [
        c"gcinfo".as_ptr(),
        c"getfenv".as_ptr(),
        c"newproxy".as_ptr(),
        c"setfenv".as_ptr(),
        c"pcall".as_ptr(),
        c"xpcall".as_ptr(),
    ];
    for name in unavailable_globals.iter() {
        LUA_PUSHCFUNCTION(vm_l, Some(unsupported_function_thunk), *name);
        lua_setglobal(vm_l, *name);
    }

    LUA_PUSHCFUNCTION(vm_l, Some(print_thunk), c"print".as_ptr());
    lua_setglobal(vm_l, c"print".as_ptr());
}
