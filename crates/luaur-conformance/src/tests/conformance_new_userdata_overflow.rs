//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2304:conformance_new_userdata_overflow`
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
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> function lua_newuserdatadtor (VM/src/lapi.cpp)
//!   - calls -> function lua_getmetatable (VM/src/lapi.cpp)
//!   - calls -> function lua_pcall (VM/src/lapi.cpp)
//!   - calls -> macro lua_tostring (VM/include/lua.h)
//!   - translates_to -> rust_item conformance_new_userdata_overflow

#[cfg(test)]
#[test]
fn conformance_new_userdata_overflow() {
    use std::ffi::CStr;

    use crate::functions::conformance_new_userdata_overflow_callback::conformance_new_userdata_overflow_callback;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_pcall::lua_pcall;
    use luaur_vm::macros::lua_pushcfunction::LUA_PUSHCFUNCTION;
    use luaur_vm::macros::lua_tostring::lua_tostring;

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        LUA_PUSHCFUNCTION(
            l,
            Some(conformance_new_userdata_overflow_callback),
            core::ptr::null(),
        );

        assert_eq!(lua_pcall(l, 0, 0, 0), lua_Status::LUA_ERRRUN as i32);
        assert_eq!(
            CStr::from_ptr(lua_tostring!(l, -1)),
            c"memory allocation error: block too big"
        );
    }
}
