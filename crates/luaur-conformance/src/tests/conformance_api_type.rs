//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2816:conformance_api_type`
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
//!   - calls -> function lua_pushnumber (VM/src/lapi.cpp)
//!   - calls -> function luaL_typename (VM/src/laux.cpp)
//!   - calls -> function lua_type (VM/src/lapi.cpp)
//!   - calls -> function lua_typename (VM/src/lapi.cpp)
//!   - calls -> macro lua_newuserdata (VM/include/lua.h)
//!   - calls -> macro lua_newtable (VM/include/lua.h)
//!   - calls -> function lua_pushstring (VM/src/lapi.cpp)
//!   - calls -> function lua_setfield (VM/src/lapi.cpp)
//!   - calls -> function lua_setmetatable (VM/src/lapi.cpp)
//!   - translates_to -> rust_item conformance_api_type

#[cfg(test)]
#[test]
fn conformance_api_type() {
    use std::ffi::CStr;

    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::enums::lua_type::lua_Type;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_typename::lua_l_typename;
    use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
    use luaur_vm::functions::lua_pushstring::lua_pushstring;
    use luaur_vm::functions::lua_setfield::lua_setfield;
    use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
    use luaur_vm::functions::lua_type::lua_type;
    use luaur_vm::functions::lua_typename::lua_typename;
    use luaur_vm::macros::lua_newtable::lua_newtable;
    use luaur_vm::macros::lua_newuserdata::lua_newuserdata;

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        lua_pushnumber(l, 2.0);
        assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"number");
        assert_eq!(CStr::from_ptr(lua_l_typename(l, 1)), c"number");
        assert_eq!(lua_type(l, -1), lua_Type::LUA_TNUMBER as i32);
        assert_eq!(lua_type(l, 1), lua_Type::LUA_TNUMBER as i32);

        assert_eq!(CStr::from_ptr(lua_l_typename(l, 2)), c"no value");
        assert_eq!(lua_type(l, 2), lua_Type::LUA_TNONE as i32);
        assert_eq!(CStr::from_ptr(lua_typename(l, lua_type(l, 2))), c"no value");

        lua_newuserdata(l, 0);
        assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"userdata");
        assert_eq!(lua_type(l, -1), lua_Type::LUA_TUSERDATA as i32);

        lua_newtable(l);
        lua_pushstring(l, c"hello".as_ptr());
        lua_setfield(l, -2, c"__type".as_ptr());
        lua_setmetatable(l, -2);

        assert_eq!(CStr::from_ptr(lua_l_typename(l, -1)), c"hello");
        assert_eq!(lua_type(l, -1), lua_Type::LUA_TUSERDATA as i32);
    }
}
