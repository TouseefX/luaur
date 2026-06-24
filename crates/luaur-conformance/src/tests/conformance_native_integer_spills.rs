//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3970:conformance_native_integer_spills`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_native_integer_spills() {
    use crate::functions::run_conformance::runConformance;
    use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

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
            userdata_types: core::ptr::null(),
            libraries_with_known_members: core::ptr::null(),
            library_member_type_cb: None,
            library_member_constant_cb: None,
            disabled_builtins: core::ptr::null(),
        };

        runConformance(
            c"native_integer_spills.luau".as_ptr(),
            None,
            None,
            core::ptr::null_mut(),
            &mut copts,
            false,
            core::ptr::null_mut(),
        );
    }
}
