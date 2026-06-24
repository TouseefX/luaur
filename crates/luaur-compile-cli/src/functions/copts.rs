use crate::records::global_options::globalOptions;
use luaur_compiler::records::compile_options::CompileOptions;

pub fn copts() -> CompileOptions {
    let mut result = CompileOptions {
        optimization_level: unsafe { globalOptions.optimizationLevel },
        debug_level: unsafe { globalOptions.debugLevel },
        type_info_level: unsafe { globalOptions.typeInfoLevel },
        coverage_level: 0,
        vector_lib: unsafe { globalOptions.vectorLib },
        vector_ctor: unsafe { globalOptions.vectorCtor },
        vector_type: unsafe { globalOptions.vectorType },
        mutable_globals: core::ptr::null(),
        userdata_types: core::ptr::null(),
        libraries_with_known_members: core::ptr::null(),
        library_member_type_cb: None,
        library_member_constant_cb: None,
        disabled_builtins: core::ptr::null(),
    };

    result
}
