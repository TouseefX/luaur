//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/DirectFieldAccess.test.cpp:226:direct_field_access_multiple_fields_same_type_dispatch_independently`
//! Source: `tests/DirectFieldAccess.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/DirectFieldAccess.test.cpp
//! - source_includes:
//!   - includes -> source_file Compiler/include/Luau/Compiler.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file CodeGen/include/luacodegen.h
//!   - includes -> source_file VM/include/lualib.h
//! - incoming:
//!   - declares <- source_file tests/DirectFieldAccess.test.cpp
//! - outgoing:
//!   - calls -> function luaL_newstate (VM/src/linit.cpp)
//!   - calls -> function lua_close (VM/src/lstate.cpp)
//!   - calls -> function lua_registeruserdatadirectfieldget (VM/src/lapi.cpp)
//!   - calls -> function lua_userdatadirectfield_setnumber (VM/src/lapi.cpp)
//!   - type_ref -> record Vec2 (tests/DirectFieldAccess.test.cpp)
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> function lua_createVec2 (tests/DirectFieldAccess.test.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - calls -> function runCode (tests/DirectFieldAccess.test.cpp)
//!   - calls -> function lua_gettop (VM/src/lapi.cpp)
//!   - calls -> macro lua_tonumber (VM/include/lua.h)
//!   - translates_to -> rust_item direct_field_access_multiple_fields_same_type_dispatch_independently

#[cfg(test)]
#[test]
fn direct_field_access_multiple_fields_same_type_dispatch_independently() {
    use crate::functions::direct_field_access_create_vec_2::direct_field_access_create_vec_2;
    use crate::functions::direct_field_access_get_x_number::direct_field_access_get_x_number;
    use crate::functions::direct_field_access_get_y_number::direct_field_access_get_y_number;
    use crate::functions::direct_field_access_k_tag_vec_2::K_TAG_VEC2;
    use crate::functions::run_code::run_code;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_gettop::lua_gettop;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget;
    use luaur_vm::macros::lua_pushcfunction::LUA_PUSHCFUNCTION;
    use luaur_vm::macros::lua_setglobal::lua_setglobal;
    use luaur_vm::macros::lua_tonumber::lua_tonumber;

    let _sff = ScopedFastFlag::new(&luaur_common::FFlag::LuauDirectFieldGet, true);
    let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = state.as_ptr();

    unsafe {
        lua_registeruserdatadirectfieldget(
            l,
            K_TAG_VEC2,
            c"X".as_ptr(),
            Some(direct_field_access_get_x_number),
        );
        lua_registeruserdatadirectfieldget(
            l,
            K_TAG_VEC2,
            c"Y".as_ptr(),
            Some(direct_field_access_get_y_number),
        );

        LUA_PUSHCFUNCTION(
            l,
            Some(direct_field_access_create_vec_2),
            c"createVec2".as_ptr(),
        );
        lua_setglobal(l, c"createVec2".as_ptr());

        let status = run_code(
            l,
            r#"
        local v = createVec2(1.5, 2.5)
        return v.X, v.Y
    "#,
        );
        assert_eq!(status, lua_Status::LUA_OK as i32);
        assert_eq!(lua_gettop(l), 2);

        assert_eq!(lua_tonumber!(l, -2), 1.5);
        assert_eq!(lua_tonumber!(l, -1), 2.5);
    }
}
