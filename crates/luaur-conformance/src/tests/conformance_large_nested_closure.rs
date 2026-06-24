//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4321:conformance_large_nested_closure`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_large_nested_closure() {
    use crate::functions::run_conformance::CODEGEN;
    use crate::type_aliases::state_ref::StateRef;
    use core::ffi::{c_char, c_void};
    use luaur_code_gen::enums::code_gen_flags::CodeGenFlags;
    use luaur_code_gen::functions::compile_internal::compile_internal;
    use luaur_code_gen::functions::luau_codegen_create::luau_codegen_create;
    use luaur_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
    use luaur_code_gen::records::compilation_options::CompilationOptions;
    use luaur_compiler::functions::luau_compile::luau_compile;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
    use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
    use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
    use luaur_vm::functions::lua_resume::lua_resume;
    use luaur_vm::functions::luau_load::luau_load;
    use luaur_vm::macros::lua_tonumber::lua_tonumber;
    use std::string::String;

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    const K_COUNT: usize = 2048;

    let mut source = String::new();
    source.push_str("local function test()\n");
    source.push_str("local x = 0\n");

    for i in 0..K_COUNT {
        let n = i + 1;
        source.push_str("    function f");
        source.push_str(&format!("{}", n));
        source.push_str("() x = x + 1; return ");
        source.push_str(&format!("{}", n));
        source.push_str(" end\n");
    }

    source.push_str("    return f");
    source.push_str(&format!("{}", K_COUNT));
    source.push_str("\n");
    source.push_str("end\n");
    source.push_str("return test()()\n");

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        if CODEGEN && luau_codegen_supported() != 0 {
            luau_codegen_create(l);
        }

        lua_l_openlibs(l);
        lua_l_sandbox(l);
        lua_l_sandboxthread(l);

        let mut bytecode_size = 0usize;
        let bytecode = luau_compile(
            source.as_ptr() as *const c_char,
            source.len(),
            core::ptr::null_mut(),
            &mut bytecode_size,
        );
        assert!(!bytecode.is_null());

        let result = luau_load(
            l,
            c"=LargeNestedClosure".as_ptr(),
            bytecode,
            bytecode_size,
            0,
        );
        free(bytecode as *mut c_void);

        assert_eq!(0, result);

        if CODEGEN && luau_codegen_supported() != 0 {
            let mut native_options = CompilationOptions::default();
            native_options.flags = CodeGenFlags::CodeGen_ColdFunctions as u32;
            let _ = compile_internal(&None, l, -1, &native_options, core::ptr::null_mut());
        }

        let status = lua_resume(l, core::ptr::null_mut(), 0);
        assert_eq!(0, status);

        assert_eq!(K_COUNT as f64, lua_tonumber!(l, -1));
    }
}
