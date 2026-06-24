//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.Conformance:tests/DirectFieldAccess.test.cpp:170:direct_field_access_unregistered_tag_falls_through_to_index_metamethod`
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
//!   - calls -> function luaL_openlibs (VM/src/linit.cpp)
//!   - calls -> function lua_registeruserdatadirectfieldget (VM/src/lapi.cpp)
//!   - calls -> function lua_userdatadirectfield_setnumber (VM/src/lapi.cpp)
//!   - type_ref -> record Vec2 (tests/DirectFieldAccess.test.cpp)
//!   - calls -> function luaL_newmetatable (VM/src/laux.cpp)
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> function lua_pushnumber (VM/src/lapi.cpp)
//!   - calls -> function lua_setfield (VM/src/lapi.cpp)
//!   - calls -> function lua_setuserdatametatable (VM/src/lapi.cpp)
//!   - calls -> function lua_createVec2 (tests/DirectFieldAccess.test.cpp)
//!   - calls -> macro lua_setglobal (VM/include/lua.h)
//!   - calls -> function lua_createOtherWithMt (tests/DirectFieldAccess.test.cpp)
//!   - calls -> function runCode (tests/DirectFieldAccess.test.cpp)
//!   - calls -> function lua_gettop (VM/src/lapi.cpp)
//!   - calls -> macro lua_tonumber (VM/include/lua.h)
//!   - translates_to -> rust_item direct_field_access_unregistered_tag_falls_through_to_index_metamethod

#[cfg(test)]
#[test]
fn direct_field_access_unregistered_tag_falls_through_to_index_metamethod() {
    use crate::functions::direct_field_access_counted_get_x_number::direct_field_access_counted_get_x_number;
    use crate::functions::direct_field_access_create_other_with_mt::direct_field_access_create_other_with_mt;
    use crate::functions::direct_field_access_create_vec_2::direct_field_access_create_vec_2;
    use crate::functions::direct_field_access_handler_hit_count::direct_field_access_handler_hit_count;
    use crate::functions::direct_field_access_k_tag_other::K_TAG_OTHER;
    use crate::functions::direct_field_access_k_tag_vec_2::K_TAG_VEC2;
    use crate::functions::direct_field_access_push_minus_one::direct_field_access_push_minus_one;
    use crate::functions::direct_field_access_reset_handler_hit_count::direct_field_access_reset_handler_hit_count;
    use crate::functions::run_code::run_code;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::state_ref::StateRef;
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_gettop::lua_gettop;
    use luaur_vm::functions::lua_l_newmetatable::lua_l_newmetatable;
    use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
    use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
    use luaur_vm::functions::lua_registeruserdatadirectfieldget::lua_registeruserdatadirectfieldget;
    use luaur_vm::functions::lua_setfield::lua_setfield;
    use luaur_vm::functions::lua_setuserdatametatable::lua_setuserdatametatable;
    use luaur_vm::macros::lua_pushcfunction::LUA_PUSHCFUNCTION;
    use luaur_vm::macros::lua_setglobal::lua_setglobal;
    use luaur_vm::macros::lua_tonumber::lua_tonumber;

    let _sff = ScopedFastFlag::new(&luaur_common::FFlag::LuauDirectFieldGet, true);
    let state = StateRef::new(lua_l_newstate()).expect("lua state allocation failed");
    let l = state.as_ptr();

    unsafe {
        lua_l_openlibs(l);

        direct_field_access_reset_handler_hit_count();
        lua_registeruserdatadirectfieldget(
            l,
            K_TAG_VEC2,
            c"X".as_ptr(),
            Some(direct_field_access_counted_get_x_number),
        );

        lua_l_newmetatable(l, c"metaOther".as_ptr());
        LUA_PUSHCFUNCTION(
            l,
            Some(direct_field_access_push_minus_one),
            c"__index".as_ptr(),
        );
        lua_setfield(l, -2, c"__index".as_ptr());
        lua_setuserdatametatable(l, K_TAG_OTHER);

        LUA_PUSHCFUNCTION(
            l,
            Some(direct_field_access_create_vec_2),
            c"createVec2".as_ptr(),
        );
        lua_setglobal(l, c"createVec2".as_ptr());
        LUA_PUSHCFUNCTION(
            l,
            Some(direct_field_access_create_other_with_mt),
            c"createOther".as_ptr(),
        );
        lua_setglobal(l, c"createOther".as_ptr());

        let status = run_code(
            l,
            r#"
        local uds = {createVec2(1, 0), createOther()}
        local results = {}
        for _, v in uds do
            results[#results + 1] = v.X
        end
        return table.unpack(results)
    "#,
        );
        assert_eq!(status, lua_Status::LUA_OK as i32);
        assert_eq!(lua_gettop(l), 2);

        assert_eq!(lua_tonumber!(l, -2), 1.0);
        assert_eq!(lua_tonumber!(l, -1), -1.0);

        assert_eq!(direct_field_access_handler_hit_count(), 1);
    }
}
