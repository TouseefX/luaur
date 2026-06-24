//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4005:conformance_native_userdata`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_native_userdata() {
    use crate::functions::conformance_native_userdata_setup::conformance_native_userdata_setup;
    use crate::functions::default_codegen_options::default_codegen_options;
    use crate::functions::run_conformance::runConformance;
    use crate::methods::lowering_fixture_initialize_codegen::{
        userdata_access_bytecode_type_callback, userdata_access_callback,
        userdata_metamethod_bytecode_type_callback, userdata_metamethod_callback,
        userdata_namecall_bytecode_type_callback, userdata_namecall_callback,
        vector_access_bytecode_type_callback, vector_access_callback,
        vector_namecall_bytecode_type_callback, vector_namecall_callback,
    };
    use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

    let userdata_compile_types = [
        c"vec2".as_ptr(),
        c"color".as_ptr(),
        c"mat3".as_ptr(),
        c"vertex".as_ptr(),
        core::ptr::null(),
    ];
    let userdata_run_types = [
        c"extra".as_ptr(),
        c"color".as_ptr(),
        c"vec2".as_ptr(),
        c"mat3".as_ptr(),
        c"vertex".as_ptr(),
        core::ptr::null(),
    ];

    for use_ir_hooks in [false, true] {
        for optimization_level in 0..=2 {
            let mut copts = LuaCompileOptions {
                optimization_level,
                debug_level: 1,
                type_info_level: 1,
                coverage_level: 0,
                vector_lib: core::ptr::null(),
                vector_ctor: core::ptr::null(),
                vector_type: core::ptr::null(),
                mutable_globals: core::ptr::null(),
                userdata_types: userdata_compile_types.as_ptr(),
                libraries_with_known_members: core::ptr::null(),
                library_member_type_cb: None,
                library_member_constant_cb: None,
                disabled_builtins: core::ptr::null(),
            };
            let mut native_options = default_codegen_options();

            if use_ir_hooks {
                native_options.hooks.vector_access_bytecode_type =
                    Some(vector_access_bytecode_type_callback);
                native_options.hooks.vector_namecall_bytecode_type =
                    Some(vector_namecall_bytecode_type_callback);
                native_options.hooks.vector_access = Some(vector_access_callback);
                native_options.hooks.vector_namecall = Some(vector_namecall_callback);

                native_options.hooks.userdata_access_bytecode_type =
                    Some(userdata_access_bytecode_type_callback);
                native_options.hooks.userdata_metamethod_bytecode_type =
                    Some(userdata_metamethod_bytecode_type_callback);
                native_options.hooks.userdata_namecall_bytecode_type =
                    Some(userdata_namecall_bytecode_type_callback);
                native_options.hooks.userdata_access = Some(userdata_access_callback);
                native_options.hooks.userdata_metamethod = Some(userdata_metamethod_callback);
                native_options.hooks.userdata_namecall = Some(userdata_namecall_callback);

                native_options.userdata_types = userdata_run_types.as_ptr();
            }

            runConformance(
                c"native_userdata.luau".as_ptr(),
                Some(conformance_native_userdata_setup),
                None,
                core::ptr::null_mut(),
                &mut copts,
                false,
                &mut native_options,
            );
        }
    }
}
