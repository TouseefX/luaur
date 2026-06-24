//! Generated skeleton item.
//! Node: `cxx:Test:Luau.Conformance:tests/Conformance.test.cpp:2430:conformance_api_iter`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> function lua_rawseti (VM/src/lapi.cpp)
//!   - calls -> function lua_next (VM/src/lapi.cpp)
//!   - calls -> function lua_pushnil (VM/src/lapi.cpp)
//!   - calls -> macro lua_tonumber (VM/include/lua.h)
//!   - calls -> macro lua_pop (VM/include/lua.h)
//!   - calls -> function lua_rawiter (VM/src/lapi.cpp)
//!   - calls -> function lua_settop (VM/src/lapi.cpp)
//!   - calls -> function lua_pushvalue (VM/src/lapi.cpp)
//!   - calls -> function lua_gettop (VM/src/lapi.cpp)
//!   - calls -> function lua_checkstack (VM/src/lapi.cpp)
//!   - translates_to -> rust_item conformance_api_iter

#[cfg(test)]
#[test]
fn conformance_api_iter() {
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::functions::lua_checkstack::lua_checkstack;
    use luaur_vm::functions::lua_gettop::lua_gettop;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_next::lua_next;
    use luaur_vm::functions::lua_pushnil::lua_pushnil;
    use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
    use luaur_vm::functions::lua_pushstring::lua_pushstring;
    use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
    use luaur_vm::functions::lua_rawiter::lua_rawiter;
    use luaur_vm::functions::lua_rawsetfield::lua_rawsetfield;
    use luaur_vm::functions::lua_rawseti::lua_rawseti;
    use luaur_vm::functions::lua_setfield::lua_setfield;
    use luaur_vm::functions::lua_settop::lua_settop;
    use luaur_vm::macros::lua_newtable::lua_newtable;
    use luaur_vm::macros::lua_pop::lua_pop;
    use luaur_vm::macros::lua_tonumber::lua_tonumber;

    let global_state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = global_state.as_ptr();

    unsafe {
        lua_newtable(l);
        lua_pushnumber(l, 123.0);
        lua_setfield(l, -2, c"key".as_ptr());
        lua_pushnumber(l, 456.0);
        lua_rawsetfield(l, -2, c"key2".as_ptr());
        lua_pushstring(l, c"test".as_ptr());
        lua_rawseti(l, -2, 1);

        let mut sum1 = 0.0;
        lua_pushnil(l);
        while lua_next(l, -2) != 0 {
            sum1 += lua_tonumber!(l, -2);
            sum1 += lua_tonumber!(l, -1);
            lua_pop(l, 1);
        }
        assert_eq!(sum1, 580.0);

        let mut sum2 = 0.0;
        let mut index = 0;
        loop {
            index = lua_rawiter(l, -1, index);
            if index < 0 {
                break;
            }

            sum2 += lua_tonumber!(l, -2);
            sum2 += lua_tonumber!(l, -1);
            lua_pop(l, 2);
        }
        assert_eq!(sum2, 580.0);

        lua_settop(l, 18);
        lua_pushvalue(l, 1);

        assert_eq!(lua_gettop(l), 19);
        assert_ne!(lua_checkstack(l, 2), 0);

        let mut sum3 = 0.0;
        let mut index = 0;
        loop {
            index = lua_rawiter(l, -1, index);
            if index < 0 {
                break;
            }

            sum3 += lua_tonumber!(l, -2);
            sum3 += lua_tonumber!(l, -1);
            lua_pop(l, 2);
        }
        assert_eq!(sum3, 580.0);

        lua_pop(l, 19);
    }
}
