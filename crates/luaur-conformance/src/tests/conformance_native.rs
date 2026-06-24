//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3912:conformance_native`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_native() {
    use crate::functions::run_conformance::{runConformance, CODEGEN};
    use crate::functions::setup_native_helpers::setup_native_helpers;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_common::FFlag;
    use luaur_compiler::records::lua_compile_options::LuaCompileOptions;

    let _luau_codegen_fix_buffer_len_check =
        ScopedFastFlag::new(&FFlag::LuauCodegenFixBufferLenCheck, true);

    if unsafe { !CODEGEN } || luau_codegen_supported() == 0 {
        return;
    }

    for debug_luau_aborting_checks in [true, false] {
        for optimization_level in 0..=2 {
            let _debug_luau_aborting_checks =
                ScopedFastFlag::new(&FFlag::DebugLuauAbortingChecks, debug_luau_aborting_checks);

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
                c"native.luau".as_ptr(),
                Some(setup_native_helpers),
                None,
                core::ptr::null_mut(),
                &mut copts,
                false,
                core::ptr::null_mut(),
            );
        }
    }
}
