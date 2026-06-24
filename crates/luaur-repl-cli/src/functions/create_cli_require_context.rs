use core::ffi::{c_int, c_void};

use luaur_vm::functions::lua_insert::lua_insert;
use luaur_vm::functions::lua_newuserdatadtor::lua_newuserdatadtor;
use luaur_vm::functions::lua_settable::lua_settable;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::macros::lua_pushlightuserdata::lua_pushlightuserdata;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::copts::copts;
use crate::functions::counters_active::counters_active;
use crate::functions::counters_track::counters_track;
use crate::functions::coverage_active::coverage_active;
use crate::functions::coverage_track::coverage_track;
use crate::functions::repl_main::repl_codegen_enabled;
use crate::methods::repl_requirer_repl_requirer::repl_requirer_repl_requirer;
use crate::records::repl_requirer::ReplRequirer;

// Destructor passed to lua_newuserdatadtor, mirroring the C++ lambda that runs
// `static_cast<ReplRequirer*>(ptr)->~ReplRequirer()`.
unsafe extern "C" fn repl_requirer_dtor(ptr: *mut c_void) {
    core::ptr::drop_in_place(ptr as *mut ReplRequirer);
}

// `coverageActive` adapter: ReplRequirer expects an `extern "C" fn() -> bool`.
unsafe extern "C" fn coverage_active_cb() -> bool {
    coverage_active()
}

// `codegenEnabled`: the C++ lambda `[]() { return codegen; }`.
unsafe extern "C" fn codegen_enabled_cb() -> bool {
    repl_codegen_enabled()
}

// `countersActive` adapter.
unsafe extern "C" fn counters_active_cb() -> bool {
    counters_active()
}

// `coverageTrack` adapter: ReplRequirer expects `extern "C" fn(*mut c_void, c_int)`.
unsafe extern "C" fn coverage_track_cb(l: *mut c_void, funcindex: c_int) {
    coverage_track(l as *mut lua_State, funcindex);
}

// `countersTrack` adapter.
unsafe extern "C" fn counters_track_cb(l: *mut c_void, funcindex: c_int) {
    counters_track(l as *mut lua_State, funcindex);
}

pub unsafe fn create_cli_require_context(l: *mut lua_State) -> *mut c_void {
    let ctx = lua_newuserdatadtor(
        l,
        core::mem::size_of::<ReplRequirer>(),
        Some(repl_requirer_dtor),
    );

    if ctx.is_null() {
        luaL_error!(l, "unable to allocate ReplRequirer");
    }

    // placement-construct the ReplRequirer into the userdata buffer
    core::ptr::write(
        ctx as *mut ReplRequirer,
        repl_requirer_repl_requirer(
            Some(copts as fn() -> luaur_compiler::records::compile_options::CompileOptions),
            Some(coverage_active_cb),
            Some(codegen_enabled_cb),
            Some(coverage_track_cb),
            Some(counters_active_cb),
            Some(counters_track_cb),
        ),
    );

    // Store ReplRequirer in the registry to keep it alive for the lifetime of
    // this lua_State. Memory address is used as a key to avoid collisions.
    lua_pushlightuserdata(l as *mut c_void, ctx);
    lua_insert(l, -2);
    lua_settable(l, LUA_REGISTRYINDEX);

    ctx
}
