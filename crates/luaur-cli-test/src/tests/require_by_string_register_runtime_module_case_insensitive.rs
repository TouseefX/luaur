//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.CLI.Test:tests/RequireByString.test.cpp:597:require_by_string_register_runtime_module_case_insensitive`
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
//!   - calls -> macro lua_pushcfunction (VM/include/lua.h)
//!   - calls -> function luarequire_registermodule (Require/src/Require.cpp)
//!   - calls -> function lua_pushstring (VM/src/lapi.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> macro lua_newtable (VM/include/lua.h)
//!   - calls -> function lua_settable (VM/src/lapi.cpp)
//!   - calls -> function lua_call (VM/src/lapi.cpp)
//!   - calls -> function runCode (CLI/src/Repl.cpp)
//!   - calls -> method ReplWithPathFixture::assertOutputContainsAll (tests/RequireByString.test.cpp)
//!   - translates_to -> rust_item require_by_string_register_runtime_module_case_insensitive

#[cfg(test)]
#[test]
fn require_by_string_register_runtime_module_case_insensitive() {
    use crate::functions::run_code::run_code;
    use crate::records::repl_with_path_fixture::ReplWithPathFixture;
    use alloc::string::String;
    use luaur_require::functions::luarequire_registermodule::luarequire_registermodule;
    use luaur_vm::functions::lua_call::lua_call;
    use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
    use luaur_vm::functions::lua_pushstring::lua_pushstring;
    use luaur_vm::functions::lua_settable::lua_settable;
    use luaur_vm::macros::lua_newtable::lua_newtable;
    use luaur_vm::records::lua_state::lua_State;

    let mut fixture = ReplWithPathFixture::new();
    let l = fixture.l as *mut lua_State;

    unsafe {
        lua_pushcclosurek(
            l,
            Some(luarequire_registermodule),
            core::ptr::null(),
            0,
            None,
        );
        lua_pushstring(l, c"@test/helloworld".as_ptr());
        lua_newtable(l);
        lua_pushstring(l, c"hello".as_ptr());
        lua_pushstring(l, c"world".as_ptr());
        lua_settable(l, -3);
        lua_call(l, 2, 0);
    }

    run_code(
        l,
        &String::from("return require('@TeSt/heLLoWoRld').hello == 'world'"),
    );
    fixture.assert_output_contains_all(&[String::from("true")]);
}
