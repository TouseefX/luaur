//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:3273:conformance_interrupt`
//! Source: `tests/Conformance.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Conformance.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file Compiler/include/luacode.h
//!   - includes -> source_file CodeGen/include/luacodegen.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/ModuleResolver.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Bytecode/include/Luau/BytecodeBuilder.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file CodeGen/include/Luau/CodeGen.h
//!   - includes -> source_file CodeGen/include/Luau/BytecodeSummary.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file tests/ConformanceIrHooks.h
//! - incoming:
//!   - declares <- source_file tests/Conformance.test.cpp
//! - outgoing:
//!   - calls -> function defaultOptions (tests/Conformance.test.cpp)
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function runConformance (tests/Conformance.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function lua_callbacks (VM/src/lapi.cpp)
//!   - calls -> function lua_getinfo (VM/src/ldebug.cpp)
//!   - calls -> function currentline (VM/src/ldebug.cpp)
//!   - calls -> function lua_yield (VM/src/ldo.cpp)
//!   - calls -> function lua_newthread (VM/src/lapi.cpp)
//!   - calls -> macro lua_getglobal (VM/include/lua.h)
//!   - calls -> function lua_resume (VM/src/ldo.cpp)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method Symbol::c_str (Analysis/include/Luau/Symbol.h)
//!   - calls -> macro luaL_error (VM/include/lualib.h)
//!   - calls -> macro luaL_checkstring (VM/include/lualib.h)
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function lua_getuserdatadtor (VM/src/lapi.cpp)
//!   - calls -> function lua_setuserdatadtor (VM/src/lapi.cpp)
//!   - calls -> macro lua_pushlightuserdata (VM/include/lua.h)
//!   - calls -> function lua_tolightuserdata (VM/src/lapi.cpp)
//!   - calls -> function lua_touserdata (VM/src/lapi.cpp)
//!   - calls -> function lua_topointer (VM/src/lapi.cpp)
//!   - calls -> macro lua_newuserdata (VM/include/lua.h)
//!   - calls -> method AssemblyBuilderX64::ud2 (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function lua_newuserdatatagged (VM/src/lapi.cpp)
//!   - calls -> function lua_touserdatatagged (VM/src/lapi.cpp)
//!   - calls -> function lua_userdatatag (VM/src/lapi.cpp)
//!   - calls -> function lua_setuserdatatag (VM/src/lapi.cpp)
//!   - calls -> function lua_newuserdatadtor (VM/src/lapi.cpp)
//!   - calls -> function luaL_newmetatable (VM/src/laux.cpp)
//!   - calls -> macro luaL_getmetatable (VM/include/lualib.h)
//!   - calls -> function lua_setmetatable (VM/src/lapi.cpp)
//!   - calls -> function luaL_checkudata (VM/src/laux.cpp)
//!   - calls -> function lua_pushvalue (VM/src/lapi.cpp)
//!   - calls -> function lua_setuserdatametatable (VM/src/lapi.cpp)
//!   - calls -> function lua_getuserdatametatable (VM/src/lapi.cpp)
//!   - calls -> function lua_newuserdatataggedwithmetatable (VM/src/lapi.cpp)
//!   - calls -> method NativeModuleRef::reset (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> function ptr (Analysis/src/TypeOrPack.cpp)
//!   - calls -> function lua_newstate (VM/src/lstate.cpp)
//!   - calls -> function lua_pushlightuserdatatagged (VM/src/lapi.cpp)
//!   - calls -> function lua_lightuserdatatag (VM/src/lapi.cpp)
//!   - calls -> function lua_tolightuserdatatagged (VM/src/lapi.cpp)
//!   - calls -> function lua_setlightuserdataname (VM/src/lapi.cpp)
//!   - calls -> function lua_getlightuserdataname (VM/src/lapi.cpp)
//!   - calls -> function luaL_typename (VM/src/laux.cpp)
//!   - calls -> function lua_rawequal (VM/src/lapi.cpp)
//!   - calls -> macro lua_newtable (VM/include/lua.h)
//!   - calls -> function lua_pushinteger (VM/src/lapi.cpp)
//!   - calls -> function lua_settable (VM/src/lapi.cpp)
//!   - calls -> function lua_gettable (VM/src/lapi.cpp)
//!   - calls -> function lua_createtable (VM/src/lapi.cpp)
//!   - calls -> function lua_pushstring (VM/src/lapi.cpp)
//!   - calls -> function lua_setfield (VM/src/lapi.cpp)
//!   - calls -> function lua_pushnumber (VM/src/lapi.cpp)
//!   - calls -> function cYieldingIteratorContinuation (tests/Conformance.test.cpp)
//!   - calls -> function luaL_checkinteger (VM/src/laux.cpp)
//!   - calls -> function cYieldingIterator (tests/Conformance.test.cpp)
//!   - calls -> function setupNativeHelpers (tests/Conformance.test.cpp)
//!   - calls -> function lua_pushcclosurek (VM/src/lapi.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - calls -> function getInt64 (tests/Conformance.test.cpp)
//!   - calls -> function lua_isnumber (VM/src/lapi.cpp)
//!   - calls -> macro lua_tointeger (VM/include/lua.h)
//!   - calls -> macro luaL_typeerror (VM/include/lualib.h)
//!   - calls -> function pushInt64 (tests/Conformance.test.cpp)
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> function luaL_checknumber (VM/src/laux.cpp)
//!   - calls -> function lua_pushboolean (VM/src/lapi.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - calls -> function lua_pushlstring (VM/src/lapi.cpp)
//!   - calls -> method FeedbackVectorFixture::run (tests/FeedbackVector.test.cpp)
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - calls -> function luau_codegen_supported (CodeGen/src/lcodegen.cpp)
//!   - calls -> function setupVectorHelpers (tests/Conformance.test.cpp)
//!   - type_ref -> record CompilationOptions (CodeGen/include/Luau/CodeGenOptions.h)
//!   - calls -> function defaultCodegenOptions (tests/Conformance.test.cpp)
//!   - calls -> function vectorAccessBytecodeType (tests/ConformanceIrHooks.h)
//!   - calls -> function vectorNamecallBytecodeType (tests/ConformanceIrHooks.h)
//!   - calls -> function vectorAccess (tests/ConformanceIrHooks.h)
//!   - calls -> function vectorNamecall (tests/ConformanceIrHooks.h)
//!   - calls -> function userdataAccessBytecodeType (tests/ConformanceIrHooks.h)
//!   - calls -> function userdataMetamethodBytecodeType (tests/ConformanceIrHooks.h)
//!   - calls -> function userdataNamecallBytecodeType (tests/ConformanceIrHooks.h)
//!   - calls -> function userdataAccess (tests/ConformanceIrHooks.h)
//!   - calls -> function userdataMetamethod (tests/ConformanceIrHooks.h)
//!   - calls -> function userdataNamecall (tests/ConformanceIrHooks.h)
//!   - calls -> function setUserdataRemapper (CodeGen/src/CodeGenContext.cpp)
//!   - calls -> function setupUserdataHelpers (tests/Conformance.test.cpp)
//!   - calls -> function getOrCreateAtom (tests/Conformance.test.cpp)
//!   - calls -> function lua_registeruserdatadirectaccess (VM/src/lapi.cpp)
//!   - calls -> function vec2DirectIndex (tests/Conformance.test.cpp)
//!   - calls -> function vec2DirectNewindex (tests/Conformance.test.cpp)
//!   - calls -> function vec2DirectNamecall (tests/Conformance.test.cpp)
//!   - calls -> function vertexDirectIndex (tests/Conformance.test.cpp)
//!   - calls -> function vertexDirectNewindex (tests/Conformance.test.cpp)
//!   - calls -> function vertexDirectNamecall (tests/Conformance.test.cpp)
//!   - calls -> function matches (Analysis/include/Luau/ControlFlow.h)
//!   - calls -> function makeHugeFunctionSource (tests/Conformance.test.cpp)
//!   - calls -> method AssemblyBuilderA64::bit (CodeGen/src/AssemblyBuilderA64.cpp)
//!   - calls -> method StringWriter::space (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function bit32 (Compiler/src/BuiltinFolding.cpp)
//!   - calls -> function luau_codegen_create (CodeGen/src/lcodegen.cpp)
//!   - calls -> function luaL_openlibs (VM/src/linit.cpp)
//!   - calls -> function luaL_sandbox (VM/src/linit.cpp)
//!   - calls -> function luaL_sandboxthread (VM/src/linit.cpp)
//!   - calls -> function luau_compile (Compiler/src/lcode.cpp)
//!   - calls -> function luau_load (VM/src/lvmload.cpp)
//!   - calls -> method FeedbackVectorFixture::compile (tests/FeedbackVector.test.cpp)
//!   - calls -> macro lua_tonumber (VM/include/lua.h)
//!   - calls -> macro lua_tostring (VM/include/lua.h)
//!   - calls -> function luaC_fullgc (VM/src/lgc.cpp)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - type_ref -> record CompilationStats (CodeGen/include/Luau/CodeGen.h)
//!   - type_ref -> record CompilationResult (CodeGen/include/Luau/CodeGen.h)
//!   - type_ref -> enum CodeGenCompilationResult (CodeGen/include/Luau/CodeGen.h)
//!   - translates_to -> rust_item conformance_interrupt

#[cfg(test)]
#[test]
fn conformance_interrupt() {
    use crate::functions::conformance_interrupt_interrupt::conformance_interrupt_interrupt;
    use crate::functions::run_conformance::runConformance;
    use crate::records::conformance_interrupt_state::{
        CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS, CONFORMANCE_INTERRUPT_MODE_INFLOOP,
        CONFORMANCE_INTERRUPT_MODE_TIMEOUT, CONFORMANCE_INTERRUPT_STATE,
    };
    use core::ffi::c_int;
    use luaur_compiler::records::lua_compile_options::LuaCompileOptions;
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_callbacks::lua_callbacks;
    use luaur_vm::functions::lua_getfield::lua_getfield;
    use luaur_vm::functions::lua_l_checklstring::lua_l_checklstring;
    use luaur_vm::functions::lua_newthread::lua_newthread;
    use luaur_vm::functions::lua_resume::lua_resume;
    use luaur_vm::macros::lua_globalsindex::LUA_GLOBALSINDEX;
    use luaur_vm::macros::lua_pop::lua_pop;
    use std::ffi::{CStr, CString};

    let mut copts = LuaCompileOptions {
        optimization_level: 1,
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

    let global_state = runConformance(
        c"interrupt.luau".as_ptr(),
        None,
        None,
        core::ptr::null_mut(),
        &mut copts,
        false,
        core::ptr::null_mut(),
    );
    let l = global_state.as_ptr();

    unsafe {
        (*lua_callbacks(l)).interrupt = Some(conformance_interrupt_interrupt);

        {
            let t = lua_newthread(l);
            lua_getfield(t, LUA_GLOBALSINDEX, c"test".as_ptr());

            CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_EXPECTED_HITS);
            let mut status = lua_resume(t, core::ptr::null_mut(), 0);
            assert_eq!(status, lua_Status::LUA_YIELD as c_int);
            assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 4);

            status = lua_resume(t, core::ptr::null_mut(), 0);
            assert_eq!(status, lua_Status::LUA_OK as c_int);
            assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 22);

            lua_pop(l, 1);
        }

        for test in 1..=10 {
            let t = lua_newthread(l);
            let name = CString::new(format!("infloop{test}")).expect("name contains nul");
            lua_getfield(t, LUA_GLOBALSINDEX, name.as_ptr());

            CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_INFLOOP);
            let status = lua_resume(t, core::ptr::null_mut(), 0);
            assert_eq!(status, lua_Status::LUA_YIELD as c_int);
            assert_eq!(CONFORMANCE_INTERRUPT_STATE.index(), 11);

            lua_pop(l, 1);
        }

        CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_TIMEOUT);

        for test in 1..=6 {
            let t = lua_newthread(l);
            let name = CString::new(format!("hang{test}")).expect("name contains nul");
            lua_getfield(t, LUA_GLOBALSINDEX, name.as_ptr());

            CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_TIMEOUT);
            let status = lua_resume(t, core::ptr::null_mut(), 0);
            assert_eq!(status, lua_Status::LUA_ERRRUN as c_int);

            let mut len = 0usize;
            let error = lua_l_checklstring(t, -1, &mut len);
            let error = CStr::from_ptr(error).to_string_lossy();
            assert!(
                error.contains("timeout"),
                "expected timeout error, got {error}"
            );

            lua_pop(l, 1);
        }

        {
            let t = lua_newthread(l);
            lua_getfield(t, LUA_GLOBALSINDEX, c"hangpcall".as_ptr());

            CONFORMANCE_INTERRUPT_STATE.reset(CONFORMANCE_INTERRUPT_MODE_TIMEOUT);
            let status = lua_resume(t, core::ptr::null_mut(), 0);
            assert_eq!(status, lua_Status::LUA_OK as c_int);

            lua_pop(l, 1);
        }
    }
}
