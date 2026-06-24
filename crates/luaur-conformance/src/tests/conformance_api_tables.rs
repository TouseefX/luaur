//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2337:conformance_api_tables`
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
//!   - calls -> macro lua_newtable (VM/include/lua.h)
//!   - calls -> function lua_pushnumber (VM/src/lapi.cpp)
//!   - calls -> function lua_setfield (VM/src/lapi.cpp)
//!   - calls -> function lua_rawsetfield (VM/src/lapi.cpp)
//!   - calls -> function lua_pushstring (VM/src/lapi.cpp)
//!   - calls -> function lua_rawseti (VM/src/lapi.cpp)
//!   - calls -> macro lua_rawsetp (VM/include/lua.h)
//!   - calls -> function lua_rawsetptagged (VM/src/lapi.cpp)
//!   - calls -> function lua_gettable (VM/src/lapi.cpp)
//!   - calls -> macro lua_tonumber (VM/include/lua.h)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - calls -> function lua_getfield (VM/src/lapi.cpp)
//!   - calls -> function lua_rawgetfield (VM/src/lapi.cpp)
//!   - calls -> function lua_rawget (VM/src/lapi.cpp)
//!   - calls -> function lua_rawgeti (VM/src/lapi.cpp)
//!   - calls -> macro lua_tostring (VM/include/lua.h)
//!   - calls -> macro lua_rawgetp (VM/include/lua.h)
//!   - calls -> function lua_rawgetptagged (VM/src/lapi.cpp)
//!   - calls -> function lua_clonetable (VM/src/lapi.cpp)
//!   - calls -> function lua_cleartable (VM/src/lapi.cpp)
//!   - calls -> function lua_pushnil (VM/src/lapi.cpp)
//!   - calls -> function lua_next (VM/src/lapi.cpp)
//!   - translates_to -> rust_item conformance_api_tables

#[cfg(test)]
#[test]
fn conformance_api_tables() {
    use core::ffi::c_void;
    use std::ffi::CStr;

    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::enums::lua_type::lua_Type;
    use luaur_vm::functions::lua_cleartable::lua_cleartable;
    use luaur_vm::functions::lua_clonetable::lua_clonetable;
    use luaur_vm::functions::lua_getfield::lua_getfield;
    use luaur_vm::functions::lua_gettable::lua_gettable;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_next::lua_next;
    use luaur_vm::functions::lua_pushnil::lua_pushnil;
    use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
    use luaur_vm::functions::lua_pushstring::lua_pushstring;
    use luaur_vm::functions::lua_rawget::lua_rawget;
    use luaur_vm::functions::lua_rawgetfield::lua_rawgetfield;
    use luaur_vm::functions::lua_rawgeti::lua_rawgeti;
    use luaur_vm::functions::lua_rawgetptagged::lua_rawgetptagged;
    use luaur_vm::functions::lua_rawsetfield::lua_rawsetfield;
    use luaur_vm::functions::lua_rawseti::lua_rawseti;
    use luaur_vm::functions::lua_rawsetptagged::lua_rawsetptagged;
    use luaur_vm::functions::lua_setfield::lua_setfield;
    use luaur_vm::macros::lua_newtable::lua_newtable;
    use luaur_vm::macros::lua_pop::lua_pop;
    use luaur_vm::macros::lua_rawgetp::lua_rawgetp;
    use luaur_vm::macros::lua_rawsetp::lua_rawsetp;
    use luaur_vm::macros::lua_tonumber::lua_tonumber;
    use luaur_vm::macros::lua_tostring::lua_tostring;

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();
    let mut lu1 = 1;
    let mut lu2 = 2;
    let lu1p = (&mut lu1 as *mut i32).cast::<c_void>();
    let lu2p = (&mut lu2 as *mut i32).cast::<c_void>();

    unsafe {
        lua_newtable(l);
        lua_pushnumber(l, 123.0);
        lua_setfield(l, -2, c"key".as_ptr());
        lua_pushnumber(l, 456.0);
        lua_rawsetfield(l, -2, c"key2".as_ptr());
        lua_pushstring(l, c"key3".as_ptr());
        lua_rawseti(l, -2, 5);
        lua_pushstring(l, c"key4".as_ptr());
        lua_rawsetp!(l, -2, lu1p);
        lua_pushstring(l, c"key5".as_ptr());
        lua_rawsetptagged(l, -2, lu2p, 1);
        lua_pushstring(l, c"key6".as_ptr());
        lua_rawsetptagged(l, -2, lu2p, 2);

        lua_pushstring(l, c"key".as_ptr());
        assert_eq!(lua_gettable(l, -2), lua_Type::LUA_TNUMBER as i32);
        assert_eq!(lua_tonumber!(l, -1), 123.0);
        lua_pop(l, 1);

        assert_eq!(
            lua_getfield(l, -1, c"key".as_ptr()),
            lua_Type::LUA_TNUMBER as i32
        );
        assert_eq!(lua_tonumber!(l, -1), 123.0);
        lua_pop(l, 1);

        assert_eq!(
            lua_rawgetfield(l, -1, c"key2".as_ptr()),
            lua_Type::LUA_TNUMBER as i32
        );
        assert_eq!(lua_tonumber!(l, -1), 456.0);
        lua_pop(l, 1);

        lua_pushstring(l, c"key".as_ptr());
        assert_eq!(lua_rawget(l, -2), lua_Type::LUA_TNUMBER as i32);
        assert_eq!(lua_tonumber!(l, -1), 123.0);
        lua_pop(l, 1);

        assert_eq!(lua_rawgeti(l, -1, 5), lua_Type::LUA_TSTRING as i32);
        assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"key3");
        lua_pop(l, 1);

        assert_eq!(lua_rawgetp(l, -1, lu1p), lua_Type::LUA_TSTRING as i32);
        assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"key4");
        lua_pop(l, 1);

        assert_eq!(
            lua_rawgetptagged(l, -1, lu2p, 1),
            lua_Type::LUA_TSTRING as i32
        );
        assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"key5");
        lua_pop(l, 1);

        assert_eq!(
            lua_rawgetptagged(l, -1, lu2p, 2),
            lua_Type::LUA_TSTRING as i32
        );
        assert_eq!(CStr::from_ptr(lua_tostring!(l, -1)), c"key6");
        lua_pop(l, 1);

        assert_eq!(lua_rawgetptagged(l, -1, lu2p, 0), lua_Type::LUA_TNIL as i32);
        lua_pop(l, 1);

        lua_clonetable(l, -1);

        assert_eq!(
            lua_getfield(l, -1, c"key".as_ptr()),
            lua_Type::LUA_TNUMBER as i32
        );
        assert_eq!(lua_tonumber!(l, -1), 123.0);
        lua_pop(l, 1);

        lua_pushnumber(l, 456.0);
        lua_rawsetfield(l, -2, c"key".as_ptr());

        lua_pop(l, 1);

        assert_eq!(
            lua_getfield(l, -1, c"key".as_ptr()),
            lua_Type::LUA_TNUMBER as i32
        );
        assert_eq!(lua_tonumber!(l, -1), 123.0);
        lua_pop(l, 1);

        lua_cleartable(l, -1);
        lua_pushnil(l);
        assert_eq!(lua_next(l, -2), 0);

        lua_pop(l, 1);
    }
}
