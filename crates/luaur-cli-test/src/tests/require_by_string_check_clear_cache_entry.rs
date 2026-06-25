//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:528:require_by_string_check_clear_cache_entry`
//! Source: `tests/RequireByString.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/RequireByString.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file VM/include/lua.h
//!   - includes -> source_file VM/include/lualib.h
//!   - includes -> source_file CLI/include/Luau/ReplRequirer.h
//!   - includes -> source_file Require/include/Luau/Require.h
//!   - includes -> source_file CLI/include/Luau/FileUtils.h
//! - incoming:
//!   - declares <- source_file tests/RequireByString.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method ReplWithPathFixture::getLuauDirectory (tests/RequireByString.test.cpp)
//!   - type_ref -> enum PathType (tests/RequireByString.test.cpp)
//!   - calls -> function luaL_findtable (VM/src/laux.cpp)
//!   - calls -> function lua_getfield (VM/src/lapi.cpp)
//!   - calls -> macro lua_isnil (VM/include/lua.h)
//!   - calls -> method ReplWithPathFixture::runProtectedRequire (tests/RequireByString.test.cpp)
//!   - calls -> method ReplWithPathFixture::assertOutputContainsAll (tests/RequireByString.test.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> function luarequire_clearcacheentry (Require/src/Require.cpp)
//!   - calls -> function lua_pushstring (VM/src/lapi.cpp)
//!   - calls -> function lua_call (VM/src/lapi.cpp)
//!   - translates_to -> rust_item require_by_string_check_clear_cache_entry

#[cfg(test)]
#[test]
fn require_by_string_check_clear_cache_entry() {
    use crate::enums::path_type::PathType;
    use crate::methods::repl_with_path_fixture_get_luau_directory::repl_with_path_fixture_get_luau_directory;
    use crate::methods::repl_with_path_fixture_run_protected_require::repl_with_path_fixture_run_protected_require;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::ffi::CString;
    use alloc::string::String;
    use luaur_require::functions::luarequire_clearcacheentry::luarequire_clearcacheentry;
    use luaur_vm::enums::lua_type::lua_Type;
    use luaur_vm::functions::lua_call::lua_call;
    use luaur_vm::functions::lua_getfield::lua_getfield;
    use luaur_vm::functions::lua_l_findtable::luaL_findtable;
    use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
    use luaur_vm::functions::lua_pushstring::lua_pushstring;
    use luaur_vm::functions::lua_type::lua_type;
    use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
    use luaur_vm::records::lua_state::lua_State;

    let mut fixture = ReplWithPathFixture::new();
    let l = fixture.l as *mut lua_State;
    let relative_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Relative)
        + "/tests/require/without_config/module";
    let absolute_path = repl_with_path_fixture_get_luau_directory(&fixture, PathType::Absolute)
        + "/tests/require/without_config/module";
    let cache_key_str = format!("{}.luau", absolute_path);
    let cache_key = CString::new(cache_key_str).unwrap();

    unsafe {
        luaL_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
        lua_getfield(l, -1, cache_key.as_ptr());
        assert!(
            lua_type(l, -1) == lua_Type::LUA_TNIL as core::ffi::c_int,
            "Cache already contained module result"
        );
    }

    repl_with_path_fixture_run_protected_require(&fixture, &relative_path);

    fixture.assert_output_contains_all(&[
        String::from("true"),
        String::from("result from dependency"),
        String::from("required into module"),
    ]);

    unsafe {
        luaL_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
        lua_getfield(l, -1, cache_key.as_ptr());
        assert!(
            lua_type(l, -1) != lua_Type::LUA_TNIL as core::ffi::c_int,
            "Cache did not contain module result"
        );

        lua_pushcclosurek(
            l,
            Some(luarequire_clearcacheentry),
            core::ptr::null(),
            0,
            None,
        );
        lua_pushstring(l, cache_key.as_ptr());
        lua_call(l, 1, 0);

        luaL_findtable(l, LUA_REGISTRYINDEX, c"_MODULES".as_ptr(), 1);
        lua_getfield(l, -1, cache_key.as_ptr());
        assert!(
            lua_type(l, -1) == lua_Type::LUA_TNIL as core::ffi::c_int,
            "Cache was not cleared"
        );
    }
}
