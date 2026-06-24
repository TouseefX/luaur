use luaur_compiler::records::compile_options::CompileOptions;

pub fn copts() -> CompileOptions {
    let mut result = CompileOptions {
        optimization_level: 0,
        debug_level: 0,
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
