//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2900:conformance_api_stack`
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
//!   - type_ref -> type_alias StateRef (tests/Conformance.test.cpp)
//!   - calls -> function lua_newstate (VM/src/lstate.cpp)
//!   - calls -> function blockableRealloc (tests/Conformance.test.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function lua_newthread (VM/src/lapi.cpp)
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> function slowlyOverflowStack (tests/Conformance.test.cpp)
//!   - calls -> function lua_pcall (VM/src/lapi.cpp)
//!   - calls -> macro luaL_checkstring (VM/include/lualib.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function lua_checkstack (VM/src/lapi.cpp)
//!   - translates_to -> rust_item conformance_api_stack

#[cfg(test)]
#[test]
fn conformance_api_stack() {
    use std::ffi::CStr;

    use crate::functions::blockable_realloc::blockable_realloc;
    use crate::functions::blockable_realloc_allowed::blockableReallocAllowed;
    use crate::functions::slowly_overflow_stack::slowly_overflow_stack;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_checkstack::lua_checkstack;
    use luaur_vm::functions::lua_newstate::lua_newstate;
    use luaur_vm::functions::lua_newthread::lua_newthread;
    use luaur_vm::functions::lua_pcall::lua_pcall;
    use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
    use luaur_vm::macros::lua_pushcfunction::LUA_PUSHCFUNCTION;
    use luaur_vm::macros::luai_maxcstack::LUAI_MAXCSTACK;

    let global_state =
        StateRef::new(unsafe { lua_newstate(Some(blockable_realloc), core::ptr::null_mut()) })
            .expect("lua state allocation failed");
    let gl = global_state.as_ptr();

    unsafe {
        let l = lua_newthread(gl);

        LUA_PUSHCFUNCTION(l, Some(slowly_overflow_stack), c"foo".as_ptr());
        let result = lua_pcall(l, 0, 0, 0);
        assert_eq!(result, lua_Status::LUA_ERRRUN as i32);
        assert_eq!(
            CStr::from_ptr(luaL_checkstring!(l, -1)),
            c"stack overflow (test)"
        );
    }

    unsafe {
        let l = lua_newthread(gl);

        assert_eq!(lua_checkstack(l, 100), 1);

        blockableReallocAllowed = false;
        assert_eq!(lua_checkstack(l, 1000), 0);
        blockableReallocAllowed = true;

        assert_eq!(lua_checkstack(l, 1000), 1);

        assert_eq!(lua_checkstack(l, LUAI_MAXCSTACK * 2), 0);
    }
}
