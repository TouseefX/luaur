//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:4268:conformance_huge_constant_table`
//! Source: `tests/Conformance.test.cpp`

#[cfg(test)]
#[test]
fn conformance_huge_constant_table() {
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

    let mut source = String::from("function foo(...)\n");
    source.push_str("    local args = ...\n");
    source.push_str("    local t = args and {\n");

    for i in 0..400 {
        for k in 0..100 {
            source.push_str("call(");
            source.push_str(&format!("{}", i * 100 + k));
            source.push_str(".125), ");
        }

        source.push_str("\n        ");
    }

    source.push_str("    }\n");
    source.push_str("    return { a = 1, b = 2, c = 3 }\n");
    source.push_str("end\n");
    source.push_str("return foo().a + foo().b\n");

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
            c"=HugeConstantTable".as_ptr(),
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

        assert_eq!(3.0, lua_tonumber!(l, -1));
    }
}
