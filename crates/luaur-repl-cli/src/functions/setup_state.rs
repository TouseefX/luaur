use core::ffi::c_void;

use luaur_code_gen::functions::luau_codegen_create::luau_codegen_create;
use luaur_require::functions::luaopen_require::luaopen_require;
use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
use luaur_vm::functions::lua_l_register::lua_l_register;
use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::records::lua_l_reg::LuaLReg;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::create_cli_require_context::create_cli_require_context;
use crate::functions::lua_collectgarbage::lua_collectgarbage;
use crate::functions::lua_loadstring::lua_loadstring;
use crate::functions::repl_main::repl_codegen_enabled;
use crate::functions::require_config_init::require_config_init;

pub unsafe fn setup_state(l: *mut lua_State) {
    if repl_codegen_enabled() {
        luau_codegen_create(l);
    }

    lua_l_openlibs(l);

    // Note: a CALLGRIND build also registers {"callgrind", lua_callgrind}; the
    // upstream default (non-CALLGRIND) build registers only these two.
    let funcs: [LuaLReg; 3] = [
        LuaLReg {
            name: c"loadstring".as_ptr(),
            func: Some(lua_loadstring),
        },
        LuaLReg {
            name: c"collectgarbage".as_ptr(),
            func: Some(lua_collectgarbage),
        },
        LuaLReg {
            name: core::ptr::null(),
            func: None,
        },
    ];

    lua_pushvalue(l, LUA_GLOBALSINDEX);
    lua_l_register(l, core::ptr::null(), funcs.as_ptr());
    lua_pop(l, 1);

    let ctx = create_cli_require_context(l);
    luaopen_require(l, Some(require_config_init), ctx as *mut c_void);

    lua_l_sandbox(l);
}
