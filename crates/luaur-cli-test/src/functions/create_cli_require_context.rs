use core::ffi::{c_int, c_void};

use crate::records::repl_requirer::ReplRequirer;
use luaur_compiler::records::compile_options::CompileOptions;
use luaur_vm::functions::lua_insert::lua_insert;
use luaur_vm::functions::lua_newuserdatadtor::lua_newuserdatadtor;
use luaur_vm::functions::lua_pushlightuserdatatagged::lua_pushlightuserdatatagged;
use luaur_vm::functions::lua_settable::lua_settable;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::records::lua_state::lua_State;

unsafe extern "C" fn replrequirer_dtor(ptr: *mut c_void) {
    core::ptr::drop_in_place(ptr as *mut ReplRequirer);
}

unsafe extern "C" fn copts_shim() -> *mut CompileOptions {
    core::ptr::null_mut()
}

unsafe extern "C" fn ret_false() -> bool {
    false
}

unsafe extern "C" fn noop_coverage(_l: *mut lua_State, _funcindex: c_int) {}

pub fn create_cli_require_context(l: *mut lua_State) -> *mut c_void {
    unsafe {
        let ctx = lua_newuserdatadtor(
            l,
            core::mem::size_of::<ReplRequirer>(),
            Some(replrequirer_dtor),
        );

        if ctx.is_null() {
            luaL_error!(l, "unable to allocate ReplRequirer");
        }

        core::ptr::write(
            ctx as *mut ReplRequirer,
            ReplRequirer::repl_requirer_repl_requirer(
                copts_shim,
                ret_false,
                ret_false,
                noop_coverage,
                ret_false,
                noop_coverage,
                core::ptr::null(),
            ),
        );

        // Store ReplRequirer in the registry to keep it alive for the lifetime of
        // this lua_State. Memory address is used as a key to avoid collisions.
        lua_pushlightuserdatatagged(l, ctx, 0);
        lua_insert(l, -2);
        lua_settable(l, LUA_REGISTRYINDEX);

        ctx
    }
}
