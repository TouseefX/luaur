//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2140:conformance_interrupt_error_inspection`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function luaL_openlibs (VM/src/linit.cpp)
//!   - calls -> function luaL_sandbox (VM/src/linit.cpp)
//!   - calls -> function luaL_sandboxthread (VM/src/linit.cpp)
//!   - calls -> function luau_compile (Compiler/src/lcode.cpp)
//!   - calls -> function luau_load (VM/src/lvmload.cpp)
//!   - calls -> function lua_callbacks (VM/src/lapi.cpp)
//!   - calls -> macro luaL_error (VM/include/lualib.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function lua_resume (VM/src/ldo.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> function lua_getinfo (VM/src/ldebug.cpp)
//!   - calls -> function luau_callhook (VM/src/lvmexecute.cpp)
//!   - translates_to -> rust_item conformance_interrupt_error_inspection

#[cfg(test)]
#[test]
fn conformance_interrupt_error_inspection() {
    use crate::functions::conformance_interrupt_error_inspection_interrupt::conformance_interrupt_error_inspection_interrupt;
    use crate::functions::conformance_interrupt_inspection_hook::conformance_interrupt_inspection_hook;
    use crate::records::conformance_interrupt_error_inspection_state::CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE;
    use crate::type_aliases::state_ref::StateRef;
    use core::ffi::{c_char, c_void};
    use luaur_compiler::functions::luau_compile::luau_compile;
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_callbacks::lua_callbacks;
    use luaur_vm::functions::lua_getinfo::lua_getinfo;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
    use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
    use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
    use luaur_vm::functions::lua_resume::lua_resume;
    use luaur_vm::functions::luau_callhook::luau_callhook;
    use luaur_vm::functions::luau_load::luau_load;
    use luaur_vm::records::lua_debug::LuaDebug;

    extern "C" {
        fn free(ptr: *mut c_void);
    }

    let source = r#"
function fib(n)
    return n < 2 and 1 or fib(n - 1) + fib(n - 2)
end

fib(5)
"#;

    for target in 0..20 {
        CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE.reset(target);

        let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
        let l = global_state.as_ptr();

        unsafe {
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
                c"=InterruptErrorInspection".as_ptr(),
                bytecode,
                bytecode_size,
                0,
            );
            free(bytecode as *mut c_void);

            assert_eq!(lua_Status::LUA_OK as i32, result);

            (*lua_callbacks(l)).interrupt = Some(conformance_interrupt_error_inspection_interrupt);

            lua_resume(l, core::ptr::null_mut(), 0);

            let mut ar: LuaDebug = core::mem::zeroed();
            assert_ne!(0, lua_getinfo(l, 0, c"nsl".as_ptr(), &mut ar));

            luau_callhook(
                l,
                Some(conformance_interrupt_inspection_hook),
                core::ptr::null_mut(),
            );
        }
    }
}
