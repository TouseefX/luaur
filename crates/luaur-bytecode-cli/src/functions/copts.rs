use luaur_compiler::records::compile_options::CompileOptions;

use crate::records::global_options::globalOptions;

pub fn copts() -> CompileOptions {
    // result.optimizationLevel = globalOptions.optimizationLevel;
    // result.debugLevel = globalOptions.debugLevel;
    // result.typeInfoLevel = 1;
    let result = CompileOptions {
        optimization_level: unsafe { globalOptions.optimization_level },
        debug_level: unsafe { globalOptions.debug_level },
        type_info_level: 1,
        coverage_level: 0,
        vector_lib: core::ptr::null(),
        vector_ctor: core::ptr::null(),
        vector_type: core::ptr::null(),
        mutable_globals: core::ptr::null(),
        userdata_types: core::ptr::null(),
        libraries_with_known_members: core::ptr::null(),
        library_member_type_cb: unsafe { core::mem::zeroed() },
        library_member_constant_cb: unsafe { core::mem::zeroed() },
        disabled_builtins: core::ptr::null(),
    };

    result
}
