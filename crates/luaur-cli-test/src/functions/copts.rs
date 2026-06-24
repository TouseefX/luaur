//! Source: `CLI/src/Repl.cpp:122` (`copts`) — faithful port.
//!
//! Returns `Luau::CompileOptions` seeded from the CLI's `globalOptions`
//! (optimization/debug level), with `typeInfoLevel = 1` and `coverageLevel`
//! derived from whether coverage is active. We return the C-ABI
//! `LuaCompileOptions` (layout-identical to `Luau::CompileOptions`) since that
//! is what `luau_compile` consumes.

use crate::functions::coverage_active::coverage_active;
use crate::functions::repl_main::global_options;
use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

pub fn copts() -> LuaCompileOptions {
    let opts = global_options();

    LuaCompileOptions {
        optimization_level: opts.optimization_level,
        debug_level: opts.debug_level,
        type_info_level: 1,
        coverage_level: if coverage_active() { 2 } else { 0 },
        vector_lib: core::ptr::null(),
        vector_ctor: core::ptr::null(),
        vector_type: core::ptr::null(),
        mutable_globals: core::ptr::null(),
        userdata_types: core::ptr::null(),
        libraries_with_known_members: core::ptr::null(),
        library_member_type_cb: None,
        library_member_constant_cb: None,
        disabled_builtins: core::ptr::null(),
    }
}
