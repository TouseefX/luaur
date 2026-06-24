//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2844:conformance_api_buffer`
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
//!   - calls -> function lua_newbuffer (VM/src/lapi.cpp)
//!   - calls -> function lua_type (VM/src/lapi.cpp)
//!   - calls -> macro lua_isbuffer (VM/include/lua.h)
//!   - calls -> function lua_objlen (VM/src/lapi.cpp)
//!   - calls -> function lua_typename (VM/src/lapi.cpp)
//!   - calls -> function luaL_typename (VM/src/laux.cpp)
//!   - calls -> function lua_tobuffer (VM/src/lapi.cpp)
//!   - calls -> function luaL_checkbuffer (VM/src/laux.cpp)
//!   - calls -> function lua_topointer (VM/src/lapi.cpp)
//!   - calls -> function lua_pushvalue (VM/src/lapi.cpp)
//!   - calls -> function lua_equal (VM/src/lapi.cpp)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - translates_to -> rust_item conformance_api_buffer

#[cfg(test)]
#[test]
fn conformance_api_buffer() {
    use std::ffi::CStr;

    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::enums::lua_type::lua_Type;
    use luaur_vm::functions::lua_equal::lua_equal;
    use luaur_vm::functions::lua_l_checkbuffer::lua_l_checkbuffer;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_typename::lua_l_typename;
    use luaur_vm::functions::lua_newbuffer::lua_newbuffer;
    use luaur_vm::functions::lua_objlen::lua_objlen;
    use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
    use luaur_vm::functions::lua_tobuffer::lua_tobuffer;
    use luaur_vm::functions::lua_topointer::lua_topointer;
    use luaur_vm::functions::lua_type::lua_type;
    use luaur_vm::functions::lua_typename::lua_typename;
    use luaur_vm::macros::lua_isbuffer::lua_isbuffer;
    use luaur_vm::macros::lua_pop::lua_pop;

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        lua_newbuffer(l, 1000);

        assert_eq!(lua_type(l, -1), lua_Type::LUA_TBUFFER as i32);

        assert!(lua_isbuffer!(l, -1));
        assert_eq!(lua_objlen(l, -1), 1000);

        assert_eq!(
            CStr::from_ptr(lua_typename(l, lua_Type::LUA_TBUFFER as i32)),
            c"buffer"
        );

        assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"buffer");

        let p1 = lua_tobuffer(l, -1, core::ptr::null_mut());

        let mut len = 0usize;
        let p2 = lua_tobuffer(l, -1, &mut len);
        assert_eq!(len, 1000);
        assert_eq!(p1, p2);

        let p3 = lua_l_checkbuffer(l, -1, core::ptr::null_mut());
        assert_eq!(p1, p3);

        len = 0;
        let p4 = lua_l_checkbuffer(l, -1, &mut len);
        assert_eq!(len, 1000);
        assert_eq!(p1, p4);

        core::ptr::write_bytes(p1.cast::<u8>(), 0xab, 1000);

        assert!(!lua_topointer(l, -1).is_null());

        lua_newbuffer(l, 0);

        lua_pushvalue(l, -2);

        assert_ne!(lua_equal(l, -3, -1), 0);
        assert_eq!(lua_equal(l, -2, -1), 0);

        lua_pop(l, 1);
    }
}
